use libc::*;

pub enum _uniPrimitive_s {}
pub enum _dnnLayout_s {}

pub type dnnPrimitive_t = *mut _uniPrimitive_s;
pub type dnnLayout_t = *mut _dnnLayout_s;
pub type dnnPrimitiveAttributes_t = *mut c_void;

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum dnnError_t {
  E_SUCCESS                   =  0,
  E_INCORRECT_INPUT_PARAMETER = -1,
  E_UNEXPECTED_NULL_POINTER   = -2,
  E_MEMORY_ERROR              = -3,
  E_UNSUPPORTED_DIMENSION     = -4,
  E_UNIMPLEMENTED             = -127
}

impl dnnError_t {
  pub fn is_ok(&self) -> bool {
    match *self {
      dnnError_t::E_SUCCESS => true,
      _ => false,
    }
  }

  pub fn is_err(&self) -> bool {
    !self.is_ok()
  }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum dnnAlgorithm_t {
  dnnAlgorithmConvolutionGemm  , // GEMM based convolution
  dnnAlgorithmConvolutionDirect, // Direct convolution
  dnnAlgorithmConvolutionFFT   , // FFT based convolution
  dnnAlgorithmPoolingMax       , // Maximum pooling
  dnnAlgorithmPoolingMin       , // Minimum pooling
  dnnAlgorithmPoolingAvg         // Average pooling
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum dnnResourceType_t {
  dnnResourceSrc            = 0,
  //dnnResourceFrom           = 0,
  dnnResourceDst            = 1,
  //dnnResourceTo             = 1,
  dnnResourceFilter         = 2,
  //dnnResourceScaleShift     = 2,
  dnnResourceBias           = 3,
  dnnResourceDiffSrc        = 4,
  dnnResourceDiffFilter     = 5,
  //dnnResourceDiffScaleShift = 5,
  dnnResourceDiffBias       = 6,
  dnnResourceDiffDst        = 7,
  dnnResourceWorkspace      = 8,
  dnnResourceMultipleSrc    = 16,
  dnnResourceMultipleDst    = 24,
  dnnResourceNumber         = 32
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum dnnBorder_t {
  dnnBorderZeros          = 0x0,
  dnnBorderExtrapolation  = 0x3
}

#[link(name = "mkl_intel_lp64")]
#[link(name = "mkl_sequential")]
#[link(name = "mkl_core")]
extern "C" {
  //pub fn dnnLayoutCreate_F32() -> dnnError_t;
  //pub fn dnnLayoutCreateFromPrimitive_F32() -> dnnError_t;
  //pub fn dnnLayoutDelete_F32() -> dnnError_t;

  pub fn dnnPrimitiveAttributesCreate_F32(attributes: *mut dnnPrimitiveAttributes_t) -> dnnError_t;
  pub fn dnnPrimitiveAttributesDestroy_F32(attributes: dnnPrimitiveAttributes_t) -> dnnError_t;
  //pub fn dnnPrimitiveGetAttributes_F32() -> dnnError_t;

  pub fn dnnExecute_F32(primitive: dnnPrimitive_t, resources: *mut *mut c_void) -> dnnError_t;
  pub fn dnnExecuteAsync_F32(primitive: dnnPrimitive_t, resources: *mut *mut c_void) -> dnnError_t;
  pub fn dnnWaitFor_F32(primitive: dnnPrimitive_t) -> dnnError_t;
  pub fn dnnDelete_F32(primitive: dnnPrimitive_t) -> dnnError_t;

  pub fn dnnConvolutionCreateForward_F32(
      p_convolution: *mut dnnPrimitive_t,
      attributes: dnnPrimitiveAttributes_t,
      algorithm: dnnAlgorithm_t,
      dimension: usize,
      src_size: *const usize,
      dst_size: *const usize,
      filter_size: *const usize,
      convolution_strides: *const usize,
      input_offset: *const c_int,
      border_type: dnnBorder_t,
      ) -> dnnError_t;
  pub fn dnnConvolutionCreateForwardBias_F32(
      p_convolution: *mut dnnPrimitive_t,
      attributes: dnnPrimitiveAttributes_t,
      algorithm: dnnAlgorithm_t,
      dimension: usize,
      src_size: *const usize,
      dst_size: *const usize,
      filter_size: *const usize,
      convolution_strides: *const usize,
      input_offset: *const c_int,
      border_type: dnnBorder_t,
      ) -> dnnError_t;
  pub fn dnnConvolutionCreateBackwardData_F32(
      p_convolution: *mut dnnPrimitive_t,
      attributes: dnnPrimitiveAttributes_t,
      algorithm: dnnAlgorithm_t,
      dimension: usize,
      src_size: *const usize,
      dst_size: *const usize,
      filter_size: *const usize,
      convolution_strides: *const usize,
      input_offset: *const c_int,
      border_type: dnnBorder_t,
      ) -> dnnError_t;
  pub fn dnnConvolutionCreateBackwardFilter_F32(
      p_convolution: *mut dnnPrimitive_t,
      attributes: dnnPrimitiveAttributes_t,
      algorithm: dnnAlgorithm_t,
      dimension: usize,
      src_size: *const usize,
      dst_size: *const usize,
      filter_size: *const usize,
      convolution_strides: *const usize,
      input_offset: *const c_int,
      border_type: dnnBorder_t,
      ) -> dnnError_t;
  pub fn dnnConvolutionCreateBackwardBias_F32(
      p_convolution: *mut dnnPrimitive_t,
      attributes: dnnPrimitiveAttributes_t,
      algorithm: dnnAlgorithm_t,
      dimension: usize,
      dst_size: *const usize,
      ) -> dnnError_t;
}
