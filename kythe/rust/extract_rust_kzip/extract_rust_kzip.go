/*
 * Copyright 2023 The Kythe Authors. All rights reserved.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// extract_rust_kzip extracts Rust targets to a kzip.
package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"kythe.io/kythe/go/extractors/bazel"
	"kythe.io/kythe/go/util/vnameutil"

	apb "kythe.io/kythe/proto/analysis_go_proto"
	spb "kythe.io/kythe/proto/storage_go_proto"
)

var (
	corpus       = flag.String("corpus", "", "The corpus label to assign (required)")
	infoFilePath = flag.String("extraction_info", "", "The path to the .rust_extraction_info.json file (required)")
	output       = flag.String("output", "", "The desired output path for the kzip (required)")
	vnamesConfig = flag.String("vnames_config", "", "The location of the VNames configuration file (required)")
)

func main() {
	flag.Parse()
	if *corpus == "" || *infoFilePath == "" || *output == "" || *vnamesConfig == "" {
		log.Fatalf("Usage: %[1]s -corpus <CORPUS> -extraction_info <PATH> -output <PATH> -vnames_config <PATH>", filepath.Base(os.Args[0]))
	}

	// Parse the extraction info
	infoContent, err := os.ReadFile(*infoFilePath)
	if err != nil {
		log.Fatalf("failed to read extraction info file: %v", err)
	}
	var info extractionInfo
	err = json.Unmarshal(infoContent, &info)
	if err != nil {
		log.Fatalf("failed to unmarshal extraction info file: %v", err)
	}

	// Load the vname rules
	rules, err := vnameutil.LoadRules(*vnamesConfig)
	if err != nil {
		log.Fatalf("error loading vname rules: %v", err)
	}

	// Create the new kzip
	kzip, err := bazel.NewKZIP(*output)
	if err != nil {
		log.Fatalf("failed to create kzip: %v", err)
	}

	// Get paths for files present in the build script out_dir if there is one
	var outDirInputs []string
	if info.OutDirPath != "" {
		matches, err := filepath.Glob(fmt.Sprintf("%s/**/*", info.OutDirPath))
		if err != nil {
			log.Fatalf("failed to glob out_dir: %v", err)
		}
		for _, match := range matches {
			if strings.HasSuffix(match, ".rmeta") {
				continue
			}
			outDirInputs = append(outDirInputs, match)
		}
	}

	// Collect the paths for all required inputs
	var sourcePaths []string
	var depPaths []string
	sourcePaths = append(sourcePaths, info.RelevantSrcs...)
	sourcePaths = append(sourcePaths, outDirInputs...)
	for _, dep := range info.Deps {
		depPaths = append(depPaths, dep.RelevantSrcs...)
	}

	// Add all of the required inputs to the kzip and make the FileInput protos
	var requiredInputs []*apb.CompilationUnit_FileInput
	for _, path := range append(sourcePaths, depPaths...) {
		r, err := os.Open(path)
		if err != nil {
			log.Fatalf("failed to open %s: %v", path, err)
		}
		digest, err := kzip.AddFile(r)
		if err != nil {
			log.Fatalf("failed to add %s to kzip: %v", path, err)
		}
		requiredInputs = append(requiredInputs, &apb.CompilationUnit_FileInput{
			Info: &apb.FileInfo{
				Path:   path,
				Digest: digest,
			},
			VName: createVName(&rules, *corpus, path),
		})
	}

	// Add the Rust project file to the kzip and the required inputs
	project := info.toProject()
	projectBytes, err := json.Marshal(project)
	if err != nil {
		log.Fatalf("failed to marshal rust project: %v", err)
	}
	projectReader := bytes.NewReader(projectBytes)
	projectDigest, err := kzip.AddFile(projectReader)
	if err != nil {
		log.Fatalf("failed to add rust project file to kzip: %v", err)
	}
	requiredInputs = append(requiredInputs, &apb.CompilationUnit_FileInput{
		Info: &apb.FileInfo{
			Path:   "kythe-rust-project.json",
			Digest: projectDigest,
		},
		VName: createVName(&rules, *corpus, "kythe-rust-project.json"),
	})

	// Create the CompilationUnit and add it to the kzip
	unitVName := createVName(&rules, *corpus, info.Root)
	unitVName.Language = "rust"
	unit := &apb.CompilationUnit{
		VName:         unitVName,
		SourceFile:    sourcePaths,
		RequiredInput: requiredInputs,
		Argument:      info.Arguments,
	}
	if info.Output != nil {
		unit.OutputKey = *info.Output
	}
	_, err = kzip.AddUnit(unit, nil)
	if err != nil {
		log.Fatalf("failed to add compilation unit to kzip: %v", err)
	}

	err = kzip.Close()
	if err != nil {
		log.Fatalf("failed to close kzip: %v", err)
	}
}

func createVName(rules *vnameutil.Rules, corpus, path string) *spb.VName {
	vname, ok := rules.Apply(path)
	if !ok {
		vname = &spb.VName{
			Corpus: corpus,
			Path:   path,
		}
	} else if vname.Corpus == "" {
		vname.Corpus = corpus
	}
	return vname
}

type extractionInfo struct {
	// The arguments from the Rust compilation action if they were available during extraction.
	Arguments []string `json:"arguments"`
	// Configuration flags for rustc.
	Cfg []string `json:"cfg"`
	// The type of crate.
	CrateType string `json:"crate_type"`
	// Direct dependencies.
	Deps []extractionInfo `json:"deps"`
	// The Rust edition.
	Edition string `json:"edition"`
	// The crate's name.
	Name string `json:"name"`
	// An optional value for the OUT_DIR environment variable.
	OutDirPath string `json:"out_dir_path"`
	// The path to the output of the action if it was available during extraction. This is always a
	// nil pointer for dependencies.
	Output *string `json:"output,omitempty"`
	// A list of file paths that are under the root module. May contain non-Rust source files needed
	// for compilation.
	RelevantSrcs []string `json:"relevant_srcs"`
	// The path to the root file for the crate
	Root string `json:"root"`
	// The target architecture
	Target string `json:"target"`
}

func (i *extractionInfo) toCrate() rustProjectCrate {
	var source *rustProjectCrateSource
	env := make(map[string]string)
	if i.OutDirPath != "" {
		source = &rustProjectCrateSource{
			Include: []string{filepath.Dir(i.Root), i.OutDirPath},
			Exclude: []string{},
		}
		env["OUT_DIR"] = i.OutDirPath
	}

	return rustProjectCrate{
		DisplayName:       i.Name,
		RootModule:        i.Root,
		Edition:           i.Edition,
		Deps:              []rustProjectCrateDep{},
		IsWorkspaceMember: false,
		Source:            source,
		Cfg:               i.Cfg,
		Target:            i.Target,
		Env:               env,
		IsProcMacro:       i.CrateType == "proc-macro",
		ProcMacroPath:     "",
	}
}

func (i *extractionInfo) toProject() rustProject {
	deps := []rustProjectCrateDep{}
	crates := []rustProjectCrate{}
	for _, dep := range i.Deps {
		deps = append(deps, rustProjectCrateDep{
			Crate: uint32(len(deps)),
			Name:  dep.Name,
		})
		crates = append(crates, dep.toCrate())
	}

	mainCrate := i.toCrate()
	mainCrate.Deps = deps
	mainCrate.IsWorkspaceMember = true
	crates = append(crates, mainCrate)

	return rustProject{
		SysrootSrc: "",
		Crates:     crates,
	}
}

type rustProject struct {
	SysrootSrc string             `json:"sysroot_src"`
	Crates     []rustProjectCrate `json:"crates"`
}

type rustProjectCrate struct {
	DisplayName       string                  `json:"display_name"`
	RootModule        string                  `json:"root_module"`
	Edition           string                  `json:"edition"`
	Deps              []rustProjectCrateDep   `json:"deps"`
	IsWorkspaceMember bool                    `json:"is_workspace_member"`
	Source            *rustProjectCrateSource `json:"source,omitempty"`
	Cfg               []string                `json:"cfg"`
	Target            string                  `json:"target"`
	Env               map[string]string       `json:"env,omitempty"`
	IsProcMacro       bool                    `json:"is_proc_macro"`
	ProcMacroPath     string                  `json:"proc_macro_dylib_path,omitempty"`
}

type rustProjectCrateSource struct {
	Include []string `json:"include_dirs"`
	Exclude []string `json:"exclude_dirs"`
}

type rustProjectCrateDep struct {
	Crate uint32 `json:"crate"`
	Name  string `json:"name"`
}
