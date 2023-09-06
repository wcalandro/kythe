// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.30.0
// 	protoc        v4.22.2
// source: kythe/proto/graph_serving.proto

package graph_serving_go_proto

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	schema_go_proto "kythe.io/kythe/proto/schema_go_proto"
	storage_go_proto "kythe.io/kythe/proto/storage_go_proto"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type Edges struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Source *storage_go_proto.VName `protobuf:"bytes,1,opt,name=source,proto3" json:"source,omitempty"`
	// Types that are assignable to Entry:
	//
	//	*Edges_Index_
	//	*Edges_Edge_
	//	*Edges_Target_
	Entry isEdges_Entry `protobuf_oneof:"entry"`
}

func (x *Edges) Reset() {
	*x = Edges{}
	if protoimpl.UnsafeEnabled {
		mi := &file_kythe_proto_graph_serving_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Edges) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Edges) ProtoMessage() {}

func (x *Edges) ProtoReflect() protoreflect.Message {
	mi := &file_kythe_proto_graph_serving_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Edges.ProtoReflect.Descriptor instead.
func (*Edges) Descriptor() ([]byte, []int) {
	return file_kythe_proto_graph_serving_proto_rawDescGZIP(), []int{0}
}

func (x *Edges) GetSource() *storage_go_proto.VName {
	if x != nil {
		return x.Source
	}
	return nil
}

func (m *Edges) GetEntry() isEdges_Entry {
	if m != nil {
		return m.Entry
	}
	return nil
}

func (x *Edges) GetIndex() *Edges_Index {
	if x, ok := x.GetEntry().(*Edges_Index_); ok {
		return x.Index
	}
	return nil
}

func (x *Edges) GetEdge() *Edges_Edge {
	if x, ok := x.GetEntry().(*Edges_Edge_); ok {
		return x.Edge
	}
	return nil
}

func (x *Edges) GetTarget() *Edges_Target {
	if x, ok := x.GetEntry().(*Edges_Target_); ok {
		return x.Target
	}
	return nil
}

type isEdges_Entry interface {
	isEdges_Entry()
}

type Edges_Index_ struct {
	Index *Edges_Index `protobuf:"bytes,2,opt,name=index,proto3,oneof"`
}

type Edges_Edge_ struct {
	Edge *Edges_Edge `protobuf:"bytes,3,opt,name=edge,proto3,oneof"`
}

type Edges_Target_ struct {
	Target *Edges_Target `protobuf:"bytes,4,opt,name=target,proto3,oneof"`
}

func (*Edges_Index_) isEdges_Entry() {}

func (*Edges_Edge_) isEdges_Entry() {}

func (*Edges_Target_) isEdges_Entry() {}

type Edges_Index struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Node *schema_go_proto.Node `protobuf:"bytes,1,opt,name=node,proto3" json:"node,omitempty"`
}

func (x *Edges_Index) Reset() {
	*x = Edges_Index{}
	if protoimpl.UnsafeEnabled {
		mi := &file_kythe_proto_graph_serving_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Edges_Index) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Edges_Index) ProtoMessage() {}

func (x *Edges_Index) ProtoReflect() protoreflect.Message {
	mi := &file_kythe_proto_graph_serving_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Edges_Index.ProtoReflect.Descriptor instead.
func (*Edges_Index) Descriptor() ([]byte, []int) {
	return file_kythe_proto_graph_serving_proto_rawDescGZIP(), []int{0, 0}
}

func (x *Edges_Index) GetNode() *schema_go_proto.Node {
	if x != nil {
		return x.Node
	}
	return nil
}

type Edges_Edge struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to Kind:
	//
	//	*Edges_Edge_KytheKind
	//	*Edges_Edge_GenericKind
	Kind    isEdges_Edge_Kind       `protobuf_oneof:"kind"`
	Ordinal int32                   `protobuf:"varint,3,opt,name=ordinal,proto3" json:"ordinal,omitempty"`
	Reverse bool                    `protobuf:"varint,4,opt,name=reverse,proto3" json:"reverse,omitempty"`
	Target  *storage_go_proto.VName `protobuf:"bytes,5,opt,name=target,proto3" json:"target,omitempty"`
}

func (x *Edges_Edge) Reset() {
	*x = Edges_Edge{}
	if protoimpl.UnsafeEnabled {
		mi := &file_kythe_proto_graph_serving_proto_msgTypes[2]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Edges_Edge) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Edges_Edge) ProtoMessage() {}

func (x *Edges_Edge) ProtoReflect() protoreflect.Message {
	mi := &file_kythe_proto_graph_serving_proto_msgTypes[2]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Edges_Edge.ProtoReflect.Descriptor instead.
func (*Edges_Edge) Descriptor() ([]byte, []int) {
	return file_kythe_proto_graph_serving_proto_rawDescGZIP(), []int{0, 1}
}

func (m *Edges_Edge) GetKind() isEdges_Edge_Kind {
	if m != nil {
		return m.Kind
	}
	return nil
}

func (x *Edges_Edge) GetKytheKind() schema_go_proto.EdgeKind {
	if x, ok := x.GetKind().(*Edges_Edge_KytheKind); ok {
		return x.KytheKind
	}
	return schema_go_proto.EdgeKind(0)
}

func (x *Edges_Edge) GetGenericKind() string {
	if x, ok := x.GetKind().(*Edges_Edge_GenericKind); ok {
		return x.GenericKind
	}
	return ""
}

func (x *Edges_Edge) GetOrdinal() int32 {
	if x != nil {
		return x.Ordinal
	}
	return 0
}

func (x *Edges_Edge) GetReverse() bool {
	if x != nil {
		return x.Reverse
	}
	return false
}

func (x *Edges_Edge) GetTarget() *storage_go_proto.VName {
	if x != nil {
		return x.Target
	}
	return nil
}

type isEdges_Edge_Kind interface {
	isEdges_Edge_Kind()
}

type Edges_Edge_KytheKind struct {
	KytheKind schema_go_proto.EdgeKind `protobuf:"varint,1,opt,name=kythe_kind,json=kytheKind,proto3,enum=kythe.proto.schema.EdgeKind,oneof"`
}

type Edges_Edge_GenericKind struct {
	GenericKind string `protobuf:"bytes,2,opt,name=generic_kind,json=genericKind,proto3,oneof"`
}

func (*Edges_Edge_KytheKind) isEdges_Edge_Kind() {}

func (*Edges_Edge_GenericKind) isEdges_Edge_Kind() {}

type Edges_Target struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Node *schema_go_proto.Node `protobuf:"bytes,1,opt,name=node,proto3" json:"node,omitempty"`
}

func (x *Edges_Target) Reset() {
	*x = Edges_Target{}
	if protoimpl.UnsafeEnabled {
		mi := &file_kythe_proto_graph_serving_proto_msgTypes[3]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Edges_Target) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Edges_Target) ProtoMessage() {}

func (x *Edges_Target) ProtoReflect() protoreflect.Message {
	mi := &file_kythe_proto_graph_serving_proto_msgTypes[3]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Edges_Target.ProtoReflect.Descriptor instead.
func (*Edges_Target) Descriptor() ([]byte, []int) {
	return file_kythe_proto_graph_serving_proto_rawDescGZIP(), []int{0, 2}
}

func (x *Edges_Target) GetNode() *schema_go_proto.Node {
	if x != nil {
		return x.Node
	}
	return nil
}

var File_kythe_proto_graph_serving_proto protoreflect.FileDescriptor

var file_kythe_proto_graph_serving_proto_rawDesc = []byte{
	0x0a, 0x1f, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x72,
	0x61, 0x70, 0x68, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x6e, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x12, 0x19, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x73,
	0x65, 0x72, 0x76, 0x69, 0x6e, 0x67, 0x2e, 0x67, 0x72, 0x61, 0x70, 0x68, 0x1a, 0x18, 0x6b, 0x79,
	0x74, 0x68, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61,
	0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x19, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2f, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x2f, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x22, 0xc0, 0x04, 0x0a, 0x05, 0x45, 0x64, 0x67, 0x65, 0x73, 0x12, 0x2a, 0x0a, 0x06, 0x73,
	0x6f, 0x75, 0x72, 0x63, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x6b, 0x79,
	0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x56, 0x4e, 0x61, 0x6d, 0x65, 0x52,
	0x06, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x12, 0x3e, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78,
	0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x6e, 0x67, 0x2e, 0x67, 0x72, 0x61,
	0x70, 0x68, 0x2e, 0x45, 0x64, 0x67, 0x65, 0x73, 0x2e, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x48, 0x00,
	0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x3b, 0x0a, 0x04, 0x65, 0x64, 0x67, 0x65, 0x18,
	0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x6e, 0x67, 0x2e, 0x67, 0x72, 0x61, 0x70,
	0x68, 0x2e, 0x45, 0x64, 0x67, 0x65, 0x73, 0x2e, 0x45, 0x64, 0x67, 0x65, 0x48, 0x00, 0x52, 0x04,
	0x65, 0x64, 0x67, 0x65, 0x12, 0x41, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x04,
	0x20, 0x01, 0x28, 0x0b, 0x32, 0x27, 0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x2e, 0x73, 0x65, 0x72, 0x76, 0x69, 0x6e, 0x67, 0x2e, 0x67, 0x72, 0x61, 0x70, 0x68,
	0x2e, 0x45, 0x64, 0x67, 0x65, 0x73, 0x2e, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x48, 0x00, 0x52,
	0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x1a, 0x35, 0x0a, 0x05, 0x49, 0x6e, 0x64, 0x65, 0x78,
	0x12, 0x2c, 0x0a, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18,
	0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x73, 0x63, 0x68,
	0x65, 0x6d, 0x61, 0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x1a, 0xd2,
	0x01, 0x0a, 0x04, 0x45, 0x64, 0x67, 0x65, 0x12, 0x3d, 0x0a, 0x0a, 0x6b, 0x79, 0x74, 0x68, 0x65,
	0x5f, 0x6b, 0x69, 0x6e, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x1c, 0x2e, 0x6b, 0x79,
	0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61,
	0x2e, 0x45, 0x64, 0x67, 0x65, 0x4b, 0x69, 0x6e, 0x64, 0x48, 0x00, 0x52, 0x09, 0x6b, 0x79, 0x74,
	0x68, 0x65, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x23, 0x0a, 0x0c, 0x67, 0x65, 0x6e, 0x65, 0x72, 0x69,
	0x63, 0x5f, 0x6b, 0x69, 0x6e, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b,
	0x67, 0x65, 0x6e, 0x65, 0x72, 0x69, 0x63, 0x4b, 0x69, 0x6e, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x6f,
	0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x05, 0x52, 0x07, 0x6f, 0x72,
	0x64, 0x69, 0x6e, 0x61, 0x6c, 0x12, 0x18, 0x0a, 0x07, 0x72, 0x65, 0x76, 0x65, 0x72, 0x73, 0x65,
	0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x07, 0x72, 0x65, 0x76, 0x65, 0x72, 0x73, 0x65, 0x12,
	0x2a, 0x0a, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
	0x12, 0x2e, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x56, 0x4e,
	0x61, 0x6d, 0x65, 0x52, 0x06, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x42, 0x06, 0x0a, 0x04, 0x6b,
	0x69, 0x6e, 0x64, 0x1a, 0x36, 0x0a, 0x06, 0x54, 0x61, 0x72, 0x67, 0x65, 0x74, 0x12, 0x2c, 0x0a,
	0x04, 0x6e, 0x6f, 0x64, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x6b, 0x79,
	0x74, 0x68, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2e, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61,
	0x2e, 0x4e, 0x6f, 0x64, 0x65, 0x52, 0x04, 0x6e, 0x6f, 0x64, 0x65, 0x42, 0x07, 0x0a, 0x05, 0x65,
	0x6e, 0x74, 0x72, 0x79, 0x42, 0x4e, 0x0a, 0x1f, 0x63, 0x6f, 0x6d, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
	0x6c, 0x65, 0x2e, 0x64, 0x65, 0x76, 0x74, 0x6f, 0x6f, 0x6c, 0x73, 0x2e, 0x6b, 0x79, 0x74, 0x68,
	0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5a, 0x2b, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2e, 0x69,
	0x6f, 0x2f, 0x6b, 0x79, 0x74, 0x68, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x72,
	0x61, 0x70, 0x68, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x69, 0x6e, 0x67, 0x5f, 0x67, 0x6f, 0x5f, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_kythe_proto_graph_serving_proto_rawDescOnce sync.Once
	file_kythe_proto_graph_serving_proto_rawDescData = file_kythe_proto_graph_serving_proto_rawDesc
)

func file_kythe_proto_graph_serving_proto_rawDescGZIP() []byte {
	file_kythe_proto_graph_serving_proto_rawDescOnce.Do(func() {
		file_kythe_proto_graph_serving_proto_rawDescData = protoimpl.X.CompressGZIP(file_kythe_proto_graph_serving_proto_rawDescData)
	})
	return file_kythe_proto_graph_serving_proto_rawDescData
}

var file_kythe_proto_graph_serving_proto_msgTypes = make([]protoimpl.MessageInfo, 4)
var file_kythe_proto_graph_serving_proto_goTypes = []interface{}{
	(*Edges)(nil),                  // 0: kythe.proto.serving.graph.Edges
	(*Edges_Index)(nil),            // 1: kythe.proto.serving.graph.Edges.Index
	(*Edges_Edge)(nil),             // 2: kythe.proto.serving.graph.Edges.Edge
	(*Edges_Target)(nil),           // 3: kythe.proto.serving.graph.Edges.Target
	(*storage_go_proto.VName)(nil), // 4: kythe.proto.VName
	(*schema_go_proto.Node)(nil),   // 5: kythe.proto.schema.Node
	(schema_go_proto.EdgeKind)(0),  // 6: kythe.proto.schema.EdgeKind
}
var file_kythe_proto_graph_serving_proto_depIdxs = []int32{
	4, // 0: kythe.proto.serving.graph.Edges.source:type_name -> kythe.proto.VName
	1, // 1: kythe.proto.serving.graph.Edges.index:type_name -> kythe.proto.serving.graph.Edges.Index
	2, // 2: kythe.proto.serving.graph.Edges.edge:type_name -> kythe.proto.serving.graph.Edges.Edge
	3, // 3: kythe.proto.serving.graph.Edges.target:type_name -> kythe.proto.serving.graph.Edges.Target
	5, // 4: kythe.proto.serving.graph.Edges.Index.node:type_name -> kythe.proto.schema.Node
	6, // 5: kythe.proto.serving.graph.Edges.Edge.kythe_kind:type_name -> kythe.proto.schema.EdgeKind
	4, // 6: kythe.proto.serving.graph.Edges.Edge.target:type_name -> kythe.proto.VName
	5, // 7: kythe.proto.serving.graph.Edges.Target.node:type_name -> kythe.proto.schema.Node
	8, // [8:8] is the sub-list for method output_type
	8, // [8:8] is the sub-list for method input_type
	8, // [8:8] is the sub-list for extension type_name
	8, // [8:8] is the sub-list for extension extendee
	0, // [0:8] is the sub-list for field type_name
}

func init() { file_kythe_proto_graph_serving_proto_init() }
func file_kythe_proto_graph_serving_proto_init() {
	if File_kythe_proto_graph_serving_proto != nil {
		return
	}
	if !protoimpl.UnsafeEnabled {
		file_kythe_proto_graph_serving_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Edges); i {
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
		file_kythe_proto_graph_serving_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Edges_Index); i {
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
		file_kythe_proto_graph_serving_proto_msgTypes[2].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Edges_Edge); i {
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
		file_kythe_proto_graph_serving_proto_msgTypes[3].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Edges_Target); i {
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
	file_kythe_proto_graph_serving_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*Edges_Index_)(nil),
		(*Edges_Edge_)(nil),
		(*Edges_Target_)(nil),
	}
	file_kythe_proto_graph_serving_proto_msgTypes[2].OneofWrappers = []interface{}{
		(*Edges_Edge_KytheKind)(nil),
		(*Edges_Edge_GenericKind)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_kythe_proto_graph_serving_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   4,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_kythe_proto_graph_serving_proto_goTypes,
		DependencyIndexes: file_kythe_proto_graph_serving_proto_depIdxs,
		MessageInfos:      file_kythe_proto_graph_serving_proto_msgTypes,
	}.Build()
	File_kythe_proto_graph_serving_proto = out.File
	file_kythe_proto_graph_serving_proto_rawDesc = nil
	file_kythe_proto_graph_serving_proto_goTypes = nil
	file_kythe_proto_graph_serving_proto_depIdxs = nil
}
