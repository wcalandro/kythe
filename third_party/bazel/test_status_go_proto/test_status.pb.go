// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.30.0
// 	protoc        v4.22.2
// source: third_party/bazel/src/main/protobuf/test_status.proto

package test_status_go_proto

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

type FailedTestCasesStatus int32

const (
	FailedTestCasesStatus_FULL          FailedTestCasesStatus = 1
	FailedTestCasesStatus_PARTIAL       FailedTestCasesStatus = 2
	FailedTestCasesStatus_NOT_AVAILABLE FailedTestCasesStatus = 3
	FailedTestCasesStatus_EMPTY         FailedTestCasesStatus = 4
)

// Enum value maps for FailedTestCasesStatus.
var (
	FailedTestCasesStatus_name = map[int32]string{
		1: "FULL",
		2: "PARTIAL",
		3: "NOT_AVAILABLE",
		4: "EMPTY",
	}
	FailedTestCasesStatus_value = map[string]int32{
		"FULL":          1,
		"PARTIAL":       2,
		"NOT_AVAILABLE": 3,
		"EMPTY":         4,
	}
)

func (x FailedTestCasesStatus) Enum() *FailedTestCasesStatus {
	p := new(FailedTestCasesStatus)
	*p = x
	return p
}

func (x FailedTestCasesStatus) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (FailedTestCasesStatus) Descriptor() protoreflect.EnumDescriptor {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[0].Descriptor()
}

func (FailedTestCasesStatus) Type() protoreflect.EnumType {
	return &file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[0]
}

func (x FailedTestCasesStatus) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Do not use.
func (x *FailedTestCasesStatus) UnmarshalJSON(b []byte) error {
	num, err := protoimpl.X.UnmarshalJSONEnum(x.Descriptor(), b)
	if err != nil {
		return err
	}
	*x = FailedTestCasesStatus(num)
	return nil
}

// Deprecated: Use FailedTestCasesStatus.Descriptor instead.
func (FailedTestCasesStatus) EnumDescriptor() ([]byte, []int) {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescGZIP(), []int{0}
}

type BlazeTestStatus int32

const (
	BlazeTestStatus_NO_STATUS                   BlazeTestStatus = 0
	BlazeTestStatus_PASSED                      BlazeTestStatus = 1
	BlazeTestStatus_FLAKY                       BlazeTestStatus = 2
	BlazeTestStatus_TIMEOUT                     BlazeTestStatus = 3
	BlazeTestStatus_FAILED                      BlazeTestStatus = 4
	BlazeTestStatus_INCOMPLETE                  BlazeTestStatus = 5
	BlazeTestStatus_REMOTE_FAILURE              BlazeTestStatus = 6
	BlazeTestStatus_FAILED_TO_BUILD             BlazeTestStatus = 7
	BlazeTestStatus_BLAZE_HALTED_BEFORE_TESTING BlazeTestStatus = 8
)

// Enum value maps for BlazeTestStatus.
var (
	BlazeTestStatus_name = map[int32]string{
		0: "NO_STATUS",
		1: "PASSED",
		2: "FLAKY",
		3: "TIMEOUT",
		4: "FAILED",
		5: "INCOMPLETE",
		6: "REMOTE_FAILURE",
		7: "FAILED_TO_BUILD",
		8: "BLAZE_HALTED_BEFORE_TESTING",
	}
	BlazeTestStatus_value = map[string]int32{
		"NO_STATUS":                   0,
		"PASSED":                      1,
		"FLAKY":                       2,
		"TIMEOUT":                     3,
		"FAILED":                      4,
		"INCOMPLETE":                  5,
		"REMOTE_FAILURE":              6,
		"FAILED_TO_BUILD":             7,
		"BLAZE_HALTED_BEFORE_TESTING": 8,
	}
)

func (x BlazeTestStatus) Enum() *BlazeTestStatus {
	p := new(BlazeTestStatus)
	*p = x
	return p
}

func (x BlazeTestStatus) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (BlazeTestStatus) Descriptor() protoreflect.EnumDescriptor {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[1].Descriptor()
}

func (BlazeTestStatus) Type() protoreflect.EnumType {
	return &file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[1]
}

func (x BlazeTestStatus) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Do not use.
func (x *BlazeTestStatus) UnmarshalJSON(b []byte) error {
	num, err := protoimpl.X.UnmarshalJSONEnum(x.Descriptor(), b)
	if err != nil {
		return err
	}
	*x = BlazeTestStatus(num)
	return nil
}

// Deprecated: Use BlazeTestStatus.Descriptor instead.
func (BlazeTestStatus) EnumDescriptor() ([]byte, []int) {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescGZIP(), []int{1}
}

type TestCase_Type int32

const (
	TestCase_TEST_CASE      TestCase_Type = 0
	TestCase_TEST_SUITE     TestCase_Type = 1
	TestCase_TEST_DECORATOR TestCase_Type = 2
	TestCase_UNKNOWN        TestCase_Type = 3
)

// Enum value maps for TestCase_Type.
var (
	TestCase_Type_name = map[int32]string{
		0: "TEST_CASE",
		1: "TEST_SUITE",
		2: "TEST_DECORATOR",
		3: "UNKNOWN",
	}
	TestCase_Type_value = map[string]int32{
		"TEST_CASE":      0,
		"TEST_SUITE":     1,
		"TEST_DECORATOR": 2,
		"UNKNOWN":        3,
	}
)

func (x TestCase_Type) Enum() *TestCase_Type {
	p := new(TestCase_Type)
	*p = x
	return p
}

func (x TestCase_Type) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (TestCase_Type) Descriptor() protoreflect.EnumDescriptor {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[2].Descriptor()
}

func (TestCase_Type) Type() protoreflect.EnumType {
	return &file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[2]
}

func (x TestCase_Type) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Do not use.
func (x *TestCase_Type) UnmarshalJSON(b []byte) error {
	num, err := protoimpl.X.UnmarshalJSONEnum(x.Descriptor(), b)
	if err != nil {
		return err
	}
	*x = TestCase_Type(num)
	return nil
}

// Deprecated: Use TestCase_Type.Descriptor instead.
func (TestCase_Type) EnumDescriptor() ([]byte, []int) {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescGZIP(), []int{0, 0}
}

type TestCase_Status int32

const (
	TestCase_PASSED TestCase_Status = 0
	TestCase_FAILED TestCase_Status = 1
	TestCase_ERROR  TestCase_Status = 2
)

// Enum value maps for TestCase_Status.
var (
	TestCase_Status_name = map[int32]string{
		0: "PASSED",
		1: "FAILED",
		2: "ERROR",
	}
	TestCase_Status_value = map[string]int32{
		"PASSED": 0,
		"FAILED": 1,
		"ERROR":  2,
	}
)

func (x TestCase_Status) Enum() *TestCase_Status {
	p := new(TestCase_Status)
	*p = x
	return p
}

func (x TestCase_Status) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (TestCase_Status) Descriptor() protoreflect.EnumDescriptor {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[3].Descriptor()
}

func (TestCase_Status) Type() protoreflect.EnumType {
	return &file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes[3]
}

func (x TestCase_Status) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Do not use.
func (x *TestCase_Status) UnmarshalJSON(b []byte) error {
	num, err := protoimpl.X.UnmarshalJSONEnum(x.Descriptor(), b)
	if err != nil {
		return err
	}
	*x = TestCase_Status(num)
	return nil
}

// Deprecated: Use TestCase_Status.Descriptor instead.
func (TestCase_Status) EnumDescriptor() ([]byte, []int) {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescGZIP(), []int{0, 1}
}

type TestCase struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Child             []*TestCase      `protobuf:"bytes,1,rep,name=child" json:"child,omitempty"`
	Name              *string          `protobuf:"bytes,2,opt,name=name" json:"name,omitempty"`
	ClassName         *string          `protobuf:"bytes,3,opt,name=class_name,json=className" json:"class_name,omitempty"`
	RunDurationMillis *int64           `protobuf:"varint,4,opt,name=run_duration_millis,json=runDurationMillis" json:"run_duration_millis,omitempty"`
	Result            *string          `protobuf:"bytes,5,opt,name=result" json:"result,omitempty"`
	Type              *TestCase_Type   `protobuf:"varint,6,opt,name=type,enum=blaze.TestCase_Type" json:"type,omitempty"`
	Status            *TestCase_Status `protobuf:"varint,7,opt,name=status,enum=blaze.TestCase_Status" json:"status,omitempty"`
	Run               *bool            `protobuf:"varint,8,opt,name=run,def=1" json:"run,omitempty"`
}

// Default values for TestCase fields.
const (
	Default_TestCase_Run = bool(true)
)

func (x *TestCase) Reset() {
	*x = TestCase{}
	if protoimpl.UnsafeEnabled {
		mi := &file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TestCase) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TestCase) ProtoMessage() {}

func (x *TestCase) ProtoReflect() protoreflect.Message {
	mi := &file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TestCase.ProtoReflect.Descriptor instead.
func (*TestCase) Descriptor() ([]byte, []int) {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescGZIP(), []int{0}
}

func (x *TestCase) GetChild() []*TestCase {
	if x != nil {
		return x.Child
	}
	return nil
}

func (x *TestCase) GetName() string {
	if x != nil && x.Name != nil {
		return *x.Name
	}
	return ""
}

func (x *TestCase) GetClassName() string {
	if x != nil && x.ClassName != nil {
		return *x.ClassName
	}
	return ""
}

func (x *TestCase) GetRunDurationMillis() int64 {
	if x != nil && x.RunDurationMillis != nil {
		return *x.RunDurationMillis
	}
	return 0
}

func (x *TestCase) GetResult() string {
	if x != nil && x.Result != nil {
		return *x.Result
	}
	return ""
}

func (x *TestCase) GetType() TestCase_Type {
	if x != nil && x.Type != nil {
		return *x.Type
	}
	return TestCase_TEST_CASE
}

func (x *TestCase) GetStatus() TestCase_Status {
	if x != nil && x.Status != nil {
		return *x.Status
	}
	return TestCase_PASSED
}

func (x *TestCase) GetRun() bool {
	if x != nil && x.Run != nil {
		return *x.Run
	}
	return Default_TestCase_Run
}

type TestResultData struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Cachable             *bool                  `protobuf:"varint,1,opt,name=cachable" json:"cachable,omitempty"`
	TestPassed           *bool                  `protobuf:"varint,2,opt,name=test_passed,json=testPassed" json:"test_passed,omitempty"`
	Status               *BlazeTestStatus       `protobuf:"varint,3,opt,name=status,enum=blaze.BlazeTestStatus,def=0" json:"status,omitempty"`
	StatusDetails        *string                `protobuf:"bytes,16,opt,name=status_details,json=statusDetails" json:"status_details,omitempty"`
	FailedLogs           []string               `protobuf:"bytes,4,rep,name=failed_logs,json=failedLogs" json:"failed_logs,omitempty"`
	Warning              []string               `protobuf:"bytes,5,rep,name=warning" json:"warning,omitempty"`
	HasCoverage          *bool                  `protobuf:"varint,6,opt,name=has_coverage,json=hasCoverage" json:"has_coverage,omitempty"`
	RemotelyCached       *bool                  `protobuf:"varint,7,opt,name=remotely_cached,json=remotelyCached" json:"remotely_cached,omitempty"`
	IsRemoteStrategy     *bool                  `protobuf:"varint,8,opt,name=is_remote_strategy,json=isRemoteStrategy" json:"is_remote_strategy,omitempty"`
	TestTimes            []int64                `protobuf:"varint,9,rep,name=test_times,json=testTimes" json:"test_times,omitempty"`
	PassedLog            *string                `protobuf:"bytes,10,opt,name=passed_log,json=passedLog" json:"passed_log,omitempty"`
	TestProcessTimes     []int64                `protobuf:"varint,11,rep,name=test_process_times,json=testProcessTimes" json:"test_process_times,omitempty"`
	RunDurationMillis    *int64                 `protobuf:"varint,12,opt,name=run_duration_millis,json=runDurationMillis" json:"run_duration_millis,omitempty"`
	StartTimeMillisEpoch *int64                 `protobuf:"varint,15,opt,name=start_time_millis_epoch,json=startTimeMillisEpoch" json:"start_time_millis_epoch,omitempty"`
	TestCase             *TestCase              `protobuf:"bytes,13,opt,name=test_case,json=testCase" json:"test_case,omitempty"`
	FailedStatus         *FailedTestCasesStatus `protobuf:"varint,14,opt,name=failed_status,json=failedStatus,enum=blaze.FailedTestCasesStatus" json:"failed_status,omitempty"`
}

// Default values for TestResultData fields.
const (
	Default_TestResultData_Status = BlazeTestStatus_NO_STATUS
)

func (x *TestResultData) Reset() {
	*x = TestResultData{}
	if protoimpl.UnsafeEnabled {
		mi := &file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *TestResultData) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*TestResultData) ProtoMessage() {}

func (x *TestResultData) ProtoReflect() protoreflect.Message {
	mi := &file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use TestResultData.ProtoReflect.Descriptor instead.
func (*TestResultData) Descriptor() ([]byte, []int) {
	return file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescGZIP(), []int{1}
}

func (x *TestResultData) GetCachable() bool {
	if x != nil && x.Cachable != nil {
		return *x.Cachable
	}
	return false
}

func (x *TestResultData) GetTestPassed() bool {
	if x != nil && x.TestPassed != nil {
		return *x.TestPassed
	}
	return false
}

func (x *TestResultData) GetStatus() BlazeTestStatus {
	if x != nil && x.Status != nil {
		return *x.Status
	}
	return Default_TestResultData_Status
}

func (x *TestResultData) GetStatusDetails() string {
	if x != nil && x.StatusDetails != nil {
		return *x.StatusDetails
	}
	return ""
}

func (x *TestResultData) GetFailedLogs() []string {
	if x != nil {
		return x.FailedLogs
	}
	return nil
}

func (x *TestResultData) GetWarning() []string {
	if x != nil {
		return x.Warning
	}
	return nil
}

func (x *TestResultData) GetHasCoverage() bool {
	if x != nil && x.HasCoverage != nil {
		return *x.HasCoverage
	}
	return false
}

func (x *TestResultData) GetRemotelyCached() bool {
	if x != nil && x.RemotelyCached != nil {
		return *x.RemotelyCached
	}
	return false
}

func (x *TestResultData) GetIsRemoteStrategy() bool {
	if x != nil && x.IsRemoteStrategy != nil {
		return *x.IsRemoteStrategy
	}
	return false
}

func (x *TestResultData) GetTestTimes() []int64 {
	if x != nil {
		return x.TestTimes
	}
	return nil
}

func (x *TestResultData) GetPassedLog() string {
	if x != nil && x.PassedLog != nil {
		return *x.PassedLog
	}
	return ""
}

func (x *TestResultData) GetTestProcessTimes() []int64 {
	if x != nil {
		return x.TestProcessTimes
	}
	return nil
}

func (x *TestResultData) GetRunDurationMillis() int64 {
	if x != nil && x.RunDurationMillis != nil {
		return *x.RunDurationMillis
	}
	return 0
}

func (x *TestResultData) GetStartTimeMillisEpoch() int64 {
	if x != nil && x.StartTimeMillisEpoch != nil {
		return *x.StartTimeMillisEpoch
	}
	return 0
}

func (x *TestResultData) GetTestCase() *TestCase {
	if x != nil {
		return x.TestCase
	}
	return nil
}

func (x *TestResultData) GetFailedStatus() FailedTestCasesStatus {
	if x != nil && x.FailedStatus != nil {
		return *x.FailedStatus
	}
	return FailedTestCasesStatus_FULL
}

var File_third_party_bazel_src_main_protobuf_test_status_proto protoreflect.FileDescriptor

var file_third_party_bazel_src_main_protobuf_test_status_proto_rawDesc = []byte{
	0x0a, 0x35, 0x74, 0x68, 0x69, 0x72, 0x64, 0x5f, 0x70, 0x61, 0x72, 0x74, 0x79, 0x2f, 0x62, 0x61,
	0x7a, 0x65, 0x6c, 0x2f, 0x73, 0x72, 0x63, 0x2f, 0x6d, 0x61, 0x69, 0x6e, 0x2f, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x62, 0x6c, 0x61, 0x7a, 0x65, 0x22, 0x93,
	0x03, 0x0a, 0x08, 0x54, 0x65, 0x73, 0x74, 0x43, 0x61, 0x73, 0x65, 0x12, 0x25, 0x0a, 0x05, 0x63,
	0x68, 0x69, 0x6c, 0x64, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x62, 0x6c, 0x61,
	0x7a, 0x65, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x43, 0x61, 0x73, 0x65, 0x52, 0x05, 0x63, 0x68, 0x69,
	0x6c, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09,
	0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6c, 0x61, 0x73, 0x73, 0x5f,
	0x6e, 0x61, 0x6d, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x63, 0x6c, 0x61, 0x73,
	0x73, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x2e, 0x0a, 0x13, 0x72, 0x75, 0x6e, 0x5f, 0x64, 0x75, 0x72,
	0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x18, 0x04, 0x20, 0x01,
	0x28, 0x03, 0x52, 0x11, 0x72, 0x75, 0x6e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x4d,
	0x69, 0x6c, 0x6c, 0x69, 0x73, 0x12, 0x16, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18,
	0x05, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x28, 0x0a,
	0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x62, 0x6c,
	0x61, 0x7a, 0x65, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x43, 0x61, 0x73, 0x65, 0x2e, 0x54, 0x79, 0x70,
	0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x2e, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x62, 0x6c, 0x61, 0x7a, 0x65, 0x2e,
	0x54, 0x65, 0x73, 0x74, 0x43, 0x61, 0x73, 0x65, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52,
	0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x16, 0x0a, 0x03, 0x72, 0x75, 0x6e, 0x18, 0x08,
	0x20, 0x01, 0x28, 0x08, 0x3a, 0x04, 0x74, 0x72, 0x75, 0x65, 0x52, 0x03, 0x72, 0x75, 0x6e, 0x22,
	0x46, 0x0a, 0x04, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0d, 0x0a, 0x09, 0x54, 0x45, 0x53, 0x54, 0x5f,
	0x43, 0x41, 0x53, 0x45, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x54, 0x45, 0x53, 0x54, 0x5f, 0x53,
	0x55, 0x49, 0x54, 0x45, 0x10, 0x01, 0x12, 0x12, 0x0a, 0x0e, 0x54, 0x45, 0x53, 0x54, 0x5f, 0x44,
	0x45, 0x43, 0x4f, 0x52, 0x41, 0x54, 0x4f, 0x52, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x55, 0x4e,
	0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x03, 0x22, 0x2b, 0x0a, 0x06, 0x53, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x12, 0x0a, 0x0a, 0x06, 0x50, 0x41, 0x53, 0x53, 0x45, 0x44, 0x10, 0x00, 0x12, 0x0a, 0x0a,
	0x06, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x52, 0x52,
	0x4f, 0x52, 0x10, 0x02, 0x22, 0xa8, 0x05, 0x0a, 0x0e, 0x54, 0x65, 0x73, 0x74, 0x52, 0x65, 0x73,
	0x75, 0x6c, 0x74, 0x44, 0x61, 0x74, 0x61, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x61, 0x63, 0x68, 0x61,
	0x62, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x08, 0x63, 0x61, 0x63, 0x68, 0x61,
	0x62, 0x6c, 0x65, 0x12, 0x1f, 0x0a, 0x0b, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x70, 0x61, 0x73, 0x73,
	0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x74, 0x65, 0x73, 0x74, 0x50, 0x61,
	0x73, 0x73, 0x65, 0x64, 0x12, 0x39, 0x0a, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x03,
	0x20, 0x01, 0x28, 0x0e, 0x32, 0x16, 0x2e, 0x62, 0x6c, 0x61, 0x7a, 0x65, 0x2e, 0x42, 0x6c, 0x61,
	0x7a, 0x65, 0x54, 0x65, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x3a, 0x09, 0x4e, 0x4f,
	0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x52, 0x06, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12,
	0x25, 0x0a, 0x0e, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c,
	0x73, 0x18, 0x10, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x44,
	0x65, 0x74, 0x61, 0x69, 0x6c, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64,
	0x5f, 0x6c, 0x6f, 0x67, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x09, 0x52, 0x0a, 0x66, 0x61, 0x69,
	0x6c, 0x65, 0x64, 0x4c, 0x6f, 0x67, 0x73, 0x12, 0x18, 0x0a, 0x07, 0x77, 0x61, 0x72, 0x6e, 0x69,
	0x6e, 0x67, 0x18, 0x05, 0x20, 0x03, 0x28, 0x09, 0x52, 0x07, 0x77, 0x61, 0x72, 0x6e, 0x69, 0x6e,
	0x67, 0x12, 0x21, 0x0a, 0x0c, 0x68, 0x61, 0x73, 0x5f, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x61, 0x67,
	0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x68, 0x61, 0x73, 0x43, 0x6f, 0x76, 0x65,
	0x72, 0x61, 0x67, 0x65, 0x12, 0x27, 0x0a, 0x0f, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x6c, 0x79,
	0x5f, 0x63, 0x61, 0x63, 0x68, 0x65, 0x64, 0x18, 0x07, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0e, 0x72,
	0x65, 0x6d, 0x6f, 0x74, 0x65, 0x6c, 0x79, 0x43, 0x61, 0x63, 0x68, 0x65, 0x64, 0x12, 0x2c, 0x0a,
	0x12, 0x69, 0x73, 0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x73, 0x74, 0x72, 0x61, 0x74,
	0x65, 0x67, 0x79, 0x18, 0x08, 0x20, 0x01, 0x28, 0x08, 0x52, 0x10, 0x69, 0x73, 0x52, 0x65, 0x6d,
	0x6f, 0x74, 0x65, 0x53, 0x74, 0x72, 0x61, 0x74, 0x65, 0x67, 0x79, 0x12, 0x1d, 0x0a, 0x0a, 0x74,
	0x65, 0x73, 0x74, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x18, 0x09, 0x20, 0x03, 0x28, 0x03, 0x52,
	0x09, 0x74, 0x65, 0x73, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x12, 0x1d, 0x0a, 0x0a, 0x70, 0x61,
	0x73, 0x73, 0x65, 0x64, 0x5f, 0x6c, 0x6f, 0x67, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x09, 0x52, 0x09,
	0x70, 0x61, 0x73, 0x73, 0x65, 0x64, 0x4c, 0x6f, 0x67, 0x12, 0x2c, 0x0a, 0x12, 0x74, 0x65, 0x73,
	0x74, 0x5f, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x18,
	0x0b, 0x20, 0x03, 0x28, 0x03, 0x52, 0x10, 0x74, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x63, 0x65,
	0x73, 0x73, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x12, 0x2e, 0x0a, 0x13, 0x72, 0x75, 0x6e, 0x5f, 0x64,
	0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x18, 0x0c,
	0x20, 0x01, 0x28, 0x03, 0x52, 0x11, 0x72, 0x75, 0x6e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f,
	0x6e, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x12, 0x35, 0x0a, 0x17, 0x73, 0x74, 0x61, 0x72, 0x74,
	0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x6d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x5f, 0x65, 0x70, 0x6f,
	0x63, 0x68, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x03, 0x52, 0x14, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54,
	0x69, 0x6d, 0x65, 0x4d, 0x69, 0x6c, 0x6c, 0x69, 0x73, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x2c,
	0x0a, 0x09, 0x74, 0x65, 0x73, 0x74, 0x5f, 0x63, 0x61, 0x73, 0x65, 0x18, 0x0d, 0x20, 0x01, 0x28,
	0x0b, 0x32, 0x0f, 0x2e, 0x62, 0x6c, 0x61, 0x7a, 0x65, 0x2e, 0x54, 0x65, 0x73, 0x74, 0x43, 0x61,
	0x73, 0x65, 0x52, 0x08, 0x74, 0x65, 0x73, 0x74, 0x43, 0x61, 0x73, 0x65, 0x12, 0x41, 0x0a, 0x0d,
	0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x18, 0x0e, 0x20,
	0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x62, 0x6c, 0x61, 0x7a, 0x65, 0x2e, 0x46, 0x61, 0x69, 0x6c,
	0x65, 0x64, 0x54, 0x65, 0x73, 0x74, 0x43, 0x61, 0x73, 0x65, 0x73, 0x53, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x52, 0x0c, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2a,
	0x4c, 0x0a, 0x15, 0x46, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x54, 0x65, 0x73, 0x74, 0x43, 0x61, 0x73,
	0x65, 0x73, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x12, 0x08, 0x0a, 0x04, 0x46, 0x55, 0x4c, 0x4c,
	0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x50, 0x41, 0x52, 0x54, 0x49, 0x41, 0x4c, 0x10, 0x02, 0x12,
	0x11, 0x0a, 0x0d, 0x4e, 0x4f, 0x54, 0x5f, 0x41, 0x56, 0x41, 0x49, 0x4c, 0x41, 0x42, 0x4c, 0x45,
	0x10, 0x03, 0x12, 0x09, 0x0a, 0x05, 0x45, 0x4d, 0x50, 0x54, 0x59, 0x10, 0x04, 0x2a, 0xaa, 0x01,
	0x0a, 0x0f, 0x42, 0x6c, 0x61, 0x7a, 0x65, 0x54, 0x65, 0x73, 0x74, 0x53, 0x74, 0x61, 0x74, 0x75,
	0x73, 0x12, 0x0d, 0x0a, 0x09, 0x4e, 0x4f, 0x5f, 0x53, 0x54, 0x41, 0x54, 0x55, 0x53, 0x10, 0x00,
	0x12, 0x0a, 0x0a, 0x06, 0x50, 0x41, 0x53, 0x53, 0x45, 0x44, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05,
	0x46, 0x4c, 0x41, 0x4b, 0x59, 0x10, 0x02, 0x12, 0x0b, 0x0a, 0x07, 0x54, 0x49, 0x4d, 0x45, 0x4f,
	0x55, 0x54, 0x10, 0x03, 0x12, 0x0a, 0x0a, 0x06, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x10, 0x04,
	0x12, 0x0e, 0x0a, 0x0a, 0x49, 0x4e, 0x43, 0x4f, 0x4d, 0x50, 0x4c, 0x45, 0x54, 0x45, 0x10, 0x05,
	0x12, 0x12, 0x0a, 0x0e, 0x52, 0x45, 0x4d, 0x4f, 0x54, 0x45, 0x5f, 0x46, 0x41, 0x49, 0x4c, 0x55,
	0x52, 0x45, 0x10, 0x06, 0x12, 0x13, 0x0a, 0x0f, 0x46, 0x41, 0x49, 0x4c, 0x45, 0x44, 0x5f, 0x54,
	0x4f, 0x5f, 0x42, 0x55, 0x49, 0x4c, 0x44, 0x10, 0x07, 0x12, 0x1f, 0x0a, 0x1b, 0x42, 0x4c, 0x41,
	0x5a, 0x45, 0x5f, 0x48, 0x41, 0x4c, 0x54, 0x45, 0x44, 0x5f, 0x42, 0x45, 0x46, 0x4f, 0x52, 0x45,
	0x5f, 0x54, 0x45, 0x53, 0x54, 0x49, 0x4e, 0x47, 0x10, 0x08, 0x42, 0x29, 0x0a, 0x27, 0x63, 0x6f,
	0x6d, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x64, 0x65, 0x76, 0x74, 0x6f, 0x6f, 0x6c,
	0x73, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2e, 0x6c, 0x69, 0x62, 0x2e, 0x76, 0x69, 0x65, 0x77,
	0x2e, 0x74, 0x65, 0x73, 0x74,
}

var (
	file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescOnce sync.Once
	file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescData = file_third_party_bazel_src_main_protobuf_test_status_proto_rawDesc
)

func file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescGZIP() []byte {
	file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescOnce.Do(func() {
		file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescData = protoimpl.X.CompressGZIP(file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescData)
	})
	return file_third_party_bazel_src_main_protobuf_test_status_proto_rawDescData
}

var file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes = make([]protoimpl.EnumInfo, 4)
var file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_third_party_bazel_src_main_protobuf_test_status_proto_goTypes = []interface{}{
	(FailedTestCasesStatus)(0), // 0: blaze.FailedTestCasesStatus
	(BlazeTestStatus)(0),       // 1: blaze.BlazeTestStatus
	(TestCase_Type)(0),         // 2: blaze.TestCase.Type
	(TestCase_Status)(0),       // 3: blaze.TestCase.Status
	(*TestCase)(nil),           // 4: blaze.TestCase
	(*TestResultData)(nil),     // 5: blaze.TestResultData
}
var file_third_party_bazel_src_main_protobuf_test_status_proto_depIdxs = []int32{
	4, // 0: blaze.TestCase.child:type_name -> blaze.TestCase
	2, // 1: blaze.TestCase.type:type_name -> blaze.TestCase.Type
	3, // 2: blaze.TestCase.status:type_name -> blaze.TestCase.Status
	1, // 3: blaze.TestResultData.status:type_name -> blaze.BlazeTestStatus
	4, // 4: blaze.TestResultData.test_case:type_name -> blaze.TestCase
	0, // 5: blaze.TestResultData.failed_status:type_name -> blaze.FailedTestCasesStatus
	6, // [6:6] is the sub-list for method output_type
	6, // [6:6] is the sub-list for method input_type
	6, // [6:6] is the sub-list for extension type_name
	6, // [6:6] is the sub-list for extension extendee
	0, // [0:6] is the sub-list for field type_name
}

func init() { file_third_party_bazel_src_main_protobuf_test_status_proto_init() }
func file_third_party_bazel_src_main_protobuf_test_status_proto_init() {
	if File_third_party_bazel_src_main_protobuf_test_status_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TestCase); i {
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
		file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*TestResultData); i {
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
			RawDescriptor: file_third_party_bazel_src_main_protobuf_test_status_proto_rawDesc,
			NumEnums:      4,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_third_party_bazel_src_main_protobuf_test_status_proto_goTypes,
		DependencyIndexes: file_third_party_bazel_src_main_protobuf_test_status_proto_depIdxs,
		EnumInfos:         file_third_party_bazel_src_main_protobuf_test_status_proto_enumTypes,
		MessageInfos:      file_third_party_bazel_src_main_protobuf_test_status_proto_msgTypes,
	}.Build()
	File_third_party_bazel_src_main_protobuf_test_status_proto = out.File
	file_third_party_bazel_src_main_protobuf_test_status_proto_rawDesc = nil
	file_third_party_bazel_src_main_protobuf_test_status_proto_goTypes = nil
	file_third_party_bazel_src_main_protobuf_test_status_proto_depIdxs = nil
}
