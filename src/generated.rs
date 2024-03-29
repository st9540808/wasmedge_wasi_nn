// This file is automatically generated, DO NOT EDIT
//
// To regenerate this file run the `crates/witx-bindgen` command

use core::mem::MaybeUninit;

pub use crate::error::Error;
pub type Result<T, E = Error> = core::result::Result<T, E>;
pub type BufferSize = u32;
pub type NnErrno = u16;
/// No error occurred.
pub const NN_ERRNO_SUCCESS: NnErrno = 0;
/// Caller module passed an invalid argument.
pub const NN_ERRNO_INVALID_ARGUMENT: NnErrno = 1;
/// Caller module is missing a memory export.
pub const NN_ERRNO_MISSING_MEMORY: NnErrno = 2;
/// Device or resource busy.
pub const NN_ERRNO_BUSY: NnErrno = 3;
pub type TensorDimensions<'a> = &'a [u32];
pub type TensorType = u8;
pub const TENSOR_TYPE_F16: TensorType = 0;
pub const TENSOR_TYPE_F32: TensorType = 1;
pub const TENSOR_TYPE_U8: TensorType = 2;
pub const TENSOR_TYPE_I32: TensorType = 3;
pub type TensorData<'a> = &'a [u8];
#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Tensor<'a> {
    /// Describe the size of the tensor (e.g. 2x2x2x2 -> [2, 2, 2, 2]). To represent a tensor containing a single value,
    /// use `[1]` for the tensor dimensions.
    pub dimensions: TensorDimensions<'a>,
    pub r#type: TensorType,
    /// Contains the tensor data.
    pub data: TensorData<'a>,
}
pub type GraphBuilder<'a> = &'a [u8];
pub type GraphBuilderArray<'a> = &'a [GraphBuilder<'a>];
pub type Graph = u32;
pub type GraphEncoding = u8;
/// TODO document buffer order
pub const GRAPH_ENCODING_OPENVINO: GraphEncoding = 0;
pub const GRAPH_ENCODING_ONNX: GraphEncoding = 1;
pub type ExecutionTarget = u8;
pub const EXECUTION_TARGET_CPU: ExecutionTarget = 0;
pub const EXECUTION_TARGET_GPU: ExecutionTarget = 1;
pub const EXECUTION_TARGET_TPU: ExecutionTarget = 2;
pub type GraphExecutionContext = u32;
/// Load an opaque sequence of bytes to use for inference.
///
/// This allows runtime implementations to support multiple graph encoding formats. For unsupported graph encodings,
/// return `errno::inval`.
///
/// ## Parameters
///
/// * `builder` - The bytes necessary to build the graph.
/// * `encoding` - The encoding of the graph.
/// * `target` - Where to execute the graph.
pub unsafe fn load(
    builder: GraphBuilderArray,
    encoding: GraphEncoding,
    target: ExecutionTarget,
) -> Result<Graph> {
    let mut graph = MaybeUninit::uninit();
    let onnx_model = &builder[0];
    let rc = wasi_ephemeral_nn::load(
        onnx_model.as_ptr(),
        onnx_model.len(),
        encoding,
        target,
        graph.as_mut_ptr(),
    );
    if let Some(err) = Error::from_raw_error(rc) {
        Err(err)
    } else {
        Ok(graph.assume_init())
    }
}

/// TODO Functions like `describe_graph_inputs` and `describe_graph_outputs` (returning
/// an array of `$tensor_description`s) might be useful for introspecting the graph but are not yet included here.
/// Create an execution instance of a loaded graph.
/// TODO this may need to accept flags that might affect the compilation or execution of the graph.
pub unsafe fn init_execution_context(graph: Graph) -> Result<GraphExecutionContext> {
    let mut context = MaybeUninit::uninit();
    let rc = wasi_ephemeral_nn::init_execution_context(graph, context.as_mut_ptr());
    if let Some(err) = Error::from_raw_error(rc) {
        Err(err)
    } else {
        Ok(context.assume_init())
    }
}

/// Define the inputs to use for inference.
///
/// This should return an $nn_errno (TODO define) if the input tensor does not match the expected dimensions and type.
///
/// ## Parameters
///
/// * `index` - The index of the input to change.
/// * `tensor` - The tensor to set as the input.
pub unsafe fn set_input(context: GraphExecutionContext, index: u32, tensor: Tensor) -> Result<()> {
    let rc = wasi_ephemeral_nn::set_input(context, index, &tensor as *const _ as *mut _);
    if let Some(err) = Error::from_raw_error(rc) {
        Err(err)
    } else {
        Ok(())
    }
}

/// Extract the outputs after inference.
///
/// This should return an $nn_errno (TODO define) if the inference has not yet run.
///
/// ## Parameters
///
/// * `index` - The index of the output to retrieve.
/// * `out_buffer` - An out parameter to which to copy the tensor data. The caller is responsible for allocating enough memory for
///   the tensor data or an error will be returned. Currently there is no dynamic way to extract the additional
///   tensor metadata (i.e. dimension, element type) but this should be added at some point.
///
/// ## Return
///
/// * `bytes_written` - The number of bytes of tensor data written to the `$out_buffer`.
pub unsafe fn get_output(
    context: GraphExecutionContext,
    index: u32,
    out_buffer: *mut u8,
    out_buffer_max_size: BufferSize,
) -> Result<BufferSize> {
    let mut bytes_written = MaybeUninit::uninit();
    let rc = wasi_ephemeral_nn::get_output(
        context,
        index,
        out_buffer,
        out_buffer_max_size,
        bytes_written.as_mut_ptr(),
    );
    if let Some(err) = Error::from_raw_error(rc) {
        Err(err)
    } else {
        Ok(bytes_written.assume_init())
    }
}

/// Compute the inference on the given inputs (see `set_input`).
///
/// This should return an $nn_errno (TODO define) if the inputs are not all defined.
pub unsafe fn compute(context: GraphExecutionContext) -> Result<()> {
    let rc = wasi_ephemeral_nn::compute(context);
    if let Some(err) = Error::from_raw_error(rc) {
        Err(err)
    } else {
        Ok(())
    }
}

#[allow(improper_ctypes)]
pub mod wasi_ephemeral_nn {
    use super::*;
    #[link(wasm_import_module = "wasi_ephemeral_nn")]
    extern "C" {
        /// Load an opaque sequence of bytes to use for inference.
        ///
        /// This allows runtime implementations to support multiple graph encoding formats. For unsupported graph encodings,
        /// return `errno::inval`.
        pub fn load(
            builder_ptr: *const u8,
            builder_len: usize,
            encoding: GraphEncoding,
            target: ExecutionTarget,
            graph: *mut Graph,
        ) -> NnErrno;
        /// TODO Functions like `describe_graph_inputs` and `describe_graph_outputs` (returning
        /// an array of `$tensor_description`s) might be useful for introspecting the graph but are not yet included here.
        /// Create an execution instance of a loaded graph.
        /// TODO this may need to accept flags that might affect the compilation or execution of the graph.
        pub fn init_execution_context(graph: Graph, context: *mut GraphExecutionContext)
            -> NnErrno;
        /// Define the inputs to use for inference.
        ///
        /// This should return an $nn_errno (TODO define) if the input tensor does not match the expected dimensions and type.
        pub fn set_input(
            context: GraphExecutionContext,
            index: u32,
            tensor: *mut Tensor,
        ) -> NnErrno;
        /// Extract the outputs after inference.
        ///
        /// This should return an $nn_errno (TODO define) if the inference has not yet run.
        pub fn get_output(
            context: GraphExecutionContext,
            index: u32,
            out_buffer: *mut u8,
            out_buffer_max_size: BufferSize,
            bytes_written: *mut BufferSize,
        ) -> NnErrno;
        /// Compute the inference on the given inputs (see `set_input`).
        ///
        /// This should return an $nn_errno (TODO define) if the inputs are not all defined.
        pub fn compute(context: GraphExecutionContext) -> NnErrno;
    }
}
