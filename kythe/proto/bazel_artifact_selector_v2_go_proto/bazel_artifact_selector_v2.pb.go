// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.30.0
// 	protoc        v4.22.2
// source: kythe/proto/bazel_artifact_selector_v2.proto

package bazel_artifact_selector_v2_go_proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type BazelAspectArtifactSelectorStateV2 struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Files      []*BazelAspectArtifactSelectorStateV2_File            `protobuf:"bytes,1,rep,name=files,proto3" json:"files,omitempty"`
	FileSets   map[int64]*BazelAspectArtifactSelectorStateV2_FileSet `protobuf:"bytes,2,rep,name=file_sets,json=fileSets,proto3" json:"file_sets,omitempty" protobuf_key:"varint,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
	Disposed   []int64                                               `protobuf:"varint,3,rep,packed,name=disposed,proto3" json:"disposed,omitempty"`
	Pending    map[int64]string                                      `protobuf:"bytes,4,rep,name=pending,proto3" json:"pending,omitempty" protobuf_key:"varint,1,opt,name=key,proto3" protobuf_val:"bytes,2,opt,name=value,proto3"`
	FileSetIds []string                                              `protobuf:"bytes,5,rep,name=file_set_ids,json=fileSetIds,proto3" json:"file_set_ids,omitempty"`
}

func (x *BazelAspectArtifactSelectorStateV2) Reset() {
	*x = BazelAspectArtifactSelectorStateV2{}
	if protoimpl.UnsafeEnabled {
		mi := &file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BazelAspectArtifactSelectorStateV2) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BazelAspectArtifactSelectorStateV2) ProtoMessage() {}

func (x *BazelAspectArtifactSelectorStateV2) ProtoReflect() protoreflect.Message {
	mi := &file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BazelAspectArtifactSelectorStateV2.ProtoReflect.Descriptor instead.
func (*BazelAspectArtifactSelectorStateV2) Descriptor() ([]byte, []int) {
	return file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescGZIP(), []int{0}
}

func (x *BazelAspectArtifactSelectorStateV2) GetFiles() []*BazelAspectArtifactSelectorStateV2_File {
	if x != nil {
		return x.Files
	}
	return nil
}

func (x *BazelAspectArtifactSelectorStateV2) GetFileSets() map[int64]*BazelAspectArtifactSelectorStateV2_FileSet {
	if x != nil {
		return x.FileSets
	}
	return nil
}

func (x *BazelAspectArtifactSelectorStateV2) GetDisposed() []int64 {
	if x != nil {
		return x.Disposed
	}
	return nil
}

func (x *BazelAspectArtifactSelectorStateV2) GetPending() map[int64]string {
	if x != nil {
		return x.Pending
	}
	return nil
}

func (x *BazelAspectArtifactSelectorStateV2) GetFileSetIds() []string {
	if x != nil {
		return x.FileSetIds
	}
	return nil
}

type BazelAspectArtifactSelectorStateV2_File struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	LocalPath string `protobuf:"bytes,1,opt,name=local_path,json=localPath,proto3" json:"local_path,omitempty"`
	Uri       string `protobuf:"bytes,2,opt,name=uri,proto3" json:"uri,omitempty"`
}

func (x *BazelAspectArtifactSelectorStateV2_File) Reset() {
	*x = BazelAspectArtifactSelectorStateV2_File{}
	if protoimpl.UnsafeEnabled {
		mi := &file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BazelAspectArtifactSelectorStateV2_File) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BazelAspectArtifactSelectorStateV2_File) ProtoMessage() {}

func (x *BazelAspectArtifactSelectorStateV2_File) ProtoReflect() protoreflect.Message {
	mi := &file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BazelAspectArtifactSelectorStateV2_File.ProtoReflect.Descriptor instead.
func (*BazelAspectArtifactSelectorStateV2_File) Descriptor() ([]byte, []int) {
	return file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescGZIP(), []int{0, 0}
}

func (x *BazelAspectArtifactSelectorStateV2_File) GetLocalPath() string {
	if x != nil {
		return x.LocalPath
	}
	return ""
}

func (x *BazelAspectArtifactSelectorStateV2_File) GetUri() string {
	if x != nil {
		return x.Uri
	}
	return ""
}

type BazelAspectArtifactSelectorStateV2_FileSet struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Files    []uint64 `protobuf:"varint,2,rep,packed,name=files,proto3" json:"files,omitempty"`
	FileSets []int64  `protobuf:"varint,3,rep,packed,name=file_sets,json=fileSets,proto3" json:"file_sets,omitempty"`
}

func (x *BazelAspectArtifactSelectorStateV2_FileSet) Reset() {
	*x = BazelAspectArtifactSelectorStateV2_FileSet{}
	if protoimpl.UnsafeEnabled {
		mi := &file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *BazelAspectArtifactSelectorStateV2_FileSet) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*BazelAspectArtifactSelectorStateV2_FileSet) ProtoMessage() {}

func (x *BazelAspectArtifactSelectorStateV2_FileSet) ProtoReflect() protoreflect.Message {
	mi := &file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use BazelAspectArtifactSelectorStateV2_FileSet.ProtoReflect.Descriptor instead.
func (*BazelAspectArtifactSelectorStateV2_FileSet) Descriptor() ([]byte, []int) {
	return file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescGZIP(), []int{0, 1}
}

func (x *BazelAspectArtifactSelectorStateV2_FileSet) GetFiles() []uint64 {
	if x != nil {
		return x.Files
	}
	return nil
}

func (x *BazelAspectArtifactSelectorStateV2_FileSet) GetFileSets() []int64 {
	if x != nil {
		return x.FileSets
	}
	return nil
}

var File_kythe_proto_bazel_artifact_selector_v2_proto protoreflect.FileDescriptor

var file_kythe_proto_bazel_artifact_selector_v2_proto_rawDesc = []byte{
	0x0a, 0x2c, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x62, 0x61,
	0x7a, 0x65, 0x6c, 0x5f, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x5f, 0x73, 0x65, 0x6c,
	0x65, 0x63, 0x74, 0x6f, 0x72, 0x5f, 0x76, 0x32, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0b,
	0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x8b, 0x05, 0x0a, 0x22,
	0x42, 0x61, 0x7a, 0x65, 0x6c, 0x41, 0x73, 0x70, 0x65, 0x63, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66,
	0x61, 0x63, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65,
	0x56, 0x32, 0x12, 0x4a, 0x0a, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x34, 0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
	0x42, 0x61, 0x7a, 0x65, 0x6c, 0x41, 0x73, 0x70, 0x65, 0x63, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66,
	0x61, 0x63, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65,
	0x56, 0x32, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x52, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x12, 0x5a,
	0x0a, 0x09, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
	0x0b, 0x32, 0x3d, 0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e,
	0x42, 0x61, 0x7a, 0x65, 0x6c, 0x41, 0x73, 0x70, 0x65, 0x63, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66,
	0x61, 0x63, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65,
	0x56, 0x32, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79,
	0x52, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x73, 0x12, 0x1a, 0x0a, 0x08, 0x64, 0x69,
	0x73, 0x70, 0x6f, 0x73, 0x65, 0x64, 0x18, 0x03, 0x20, 0x03, 0x28, 0x03, 0x52, 0x08, 0x64, 0x69,
	0x73, 0x70, 0x6f, 0x73, 0x65, 0x64, 0x12, 0x56, 0x0a, 0x07, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e,
	0x67, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x3c, 0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e,
	0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x42, 0x61, 0x7a, 0x65, 0x6c, 0x41, 0x73, 0x70, 0x65, 0x63,
	0x74, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f,
	0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x56, 0x32, 0x2e, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67,
	0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x07, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x12, 0x20,
	0x0a, 0x0c, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x05,
	0x20, 0x03, 0x28, 0x09, 0x52, 0x0a, 0x66, 0x69, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x49, 0x64, 0x73,
	0x1a, 0x37, 0x0a, 0x04, 0x46, 0x69, 0x6c, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x6c, 0x6f, 0x63, 0x61,
	0x6c, 0x5f, 0x70, 0x61, 0x74, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x6c, 0x6f,
	0x63, 0x61, 0x6c, 0x50, 0x61, 0x74, 0x68, 0x12, 0x10, 0x0a, 0x03, 0x75, 0x72, 0x69, 0x18, 0x02,
	0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72, 0x69, 0x1a, 0x3c, 0x0a, 0x07, 0x46, 0x69, 0x6c,
	0x65, 0x53, 0x65, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x02, 0x20,
	0x03, 0x28, 0x04, 0x52, 0x05, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x12, 0x1b, 0x0a, 0x09, 0x66, 0x69,
	0x6c, 0x65, 0x5f, 0x73, 0x65, 0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x03, 0x52, 0x08, 0x66,
	0x69, 0x6c, 0x65, 0x53, 0x65, 0x74, 0x73, 0x1a, 0x74, 0x0a, 0x0d, 0x46, 0x69, 0x6c, 0x65, 0x53,
	0x65, 0x74, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
	0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x4d, 0x0a, 0x05, 0x76, 0x61,
	0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x37, 0x2e, 0x6b, 0x79, 0x74, 0x68,
	0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x42, 0x61, 0x7a, 0x65, 0x6c, 0x41, 0x73, 0x70,
	0x65, 0x63, 0x74, 0x41, 0x72, 0x74, 0x69, 0x66, 0x61, 0x63, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63,
	0x74, 0x6f, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x56, 0x32, 0x2e, 0x46, 0x69, 0x6c, 0x65, 0x53,
	0x65, 0x74, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a, 0x3a, 0x0a,
	0x0c, 0x50, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a,
	0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12,
	0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05,
	0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x42, 0x3c, 0x50, 0x01, 0x5a, 0x38, 0x6b,
	0x79, 0x74, 0x68, 0x65, 0x2e, 0x69, 0x6f, 0x2f, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2f, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x2f, 0x62, 0x61, 0x7a, 0x65, 0x6c, 0x5f, 0x61, 0x72, 0x74, 0x69, 0x66, 0x61,
	0x63, 0x74, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x5f, 0x76, 0x32, 0x5f, 0x67,
	0x6f, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescOnce sync.Once
	file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescData = file_kythe_proto_bazel_artifact_selector_v2_proto_rawDesc
)

func file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescGZIP() []byte {
	file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescOnce.Do(func() {
		file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescData = protoimpl.X.CompressGZIP(file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescData)
	})
	return file_kythe_proto_bazel_artifact_selector_v2_proto_rawDescData
}

var file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes = make([]protoimpl.MessageInfo, 5)
var file_kythe_proto_bazel_artifact_selector_v2_proto_goTypes = []interface{}{
	(*BazelAspectArtifactSelectorStateV2)(nil),         // 0: kythe.proto.BazelAspectArtifactSelectorStateV2
	(*BazelAspectArtifactSelectorStateV2_File)(nil),    // 1: kythe.proto.BazelAspectArtifactSelectorStateV2.File
	(*BazelAspectArtifactSelectorStateV2_FileSet)(nil), // 2: kythe.proto.BazelAspectArtifactSelectorStateV2.FileSet
	nil, // 3: kythe.proto.BazelAspectArtifactSelectorStateV2.FileSetsEntry
	nil, // 4: kythe.proto.BazelAspectArtifactSelectorStateV2.PendingEntry
}
var file_kythe_proto_bazel_artifact_selector_v2_proto_depIdxs = []int32{
	1, // 0: kythe.proto.BazelAspectArtifactSelectorStateV2.files:type_name -> kythe.proto.BazelAspectArtifactSelectorStateV2.File
	3, // 1: kythe.proto.BazelAspectArtifactSelectorStateV2.file_sets:type_name -> kythe.proto.BazelAspectArtifactSelectorStateV2.FileSetsEntry
	4, // 2: kythe.proto.BazelAspectArtifactSelectorStateV2.pending:type_name -> kythe.proto.BazelAspectArtifactSelectorStateV2.PendingEntry
	2, // 3: kythe.proto.BazelAspectArtifactSelectorStateV2.FileSetsEntry.value:type_name -> kythe.proto.BazelAspectArtifactSelectorStateV2.FileSet
	4, // [4:4] is the sub-list for method output_type
	4, // [4:4] is the sub-list for method input_type
	4, // [4:4] is the sub-list for extension type_name
	4, // [4:4] is the sub-list for extension extendee
	0, // [0:4] is the sub-list for field type_name
}

func init() { file_kythe_proto_bazel_artifact_selector_v2_proto_init() }
func file_kythe_proto_bazel_artifact_selector_v2_proto_init() {
	if File_kythe_proto_bazel_artifact_selector_v2_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BazelAspectArtifactSelectorStateV2); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BazelAspectArtifactSelectorStateV2_File); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*BazelAspectArtifactSelectorStateV2_FileSet); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_kythe_proto_bazel_artifact_selector_v2_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   5,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_kythe_proto_bazel_artifact_selector_v2_proto_goTypes,
		DependencyIndexes: file_kythe_proto_bazel_artifact_selector_v2_proto_depIdxs,
		MessageInfos:      file_kythe_proto_bazel_artifact_selector_v2_proto_msgTypes,
	}.Build()
	File_kythe_proto_bazel_artifact_selector_v2_proto = out.File
	file_kythe_proto_bazel_artifact_selector_v2_proto_rawDesc = nil
	file_kythe_proto_bazel_artifact_selector_v2_proto_goTypes = nil
	file_kythe_proto_bazel_artifact_selector_v2_proto_depIdxs = nil
}
