use icicle_cuda_runtime::device_context::DeviceContext;
use std::os::raw::c_int;

/**
 * @enum Ordering
 * How to order inputs and outputs of the NTT. If needed, use this field to specify decimation: decimation in time
 * (DIT) corresponds to `Ordering::kRN` while decimation in frequency (DIF) to `Ordering::kNR`. Also, to specify
 * butterfly to be used, select `Ordering::kRN` for Cooley-Tukey and `Ordering::kNR` for Gentleman-Sande. There's
 * no implication that a certain decimation or butterfly will actually be used under the hood, this is just for
 * compatibility with codebases that use "decimation" and "butterfly" to denote ordering of inputs and outputs.
 *
 * Ordering options are:
 * - kNN: inputs and outputs are natural-order (example of natural ordering: \f$ \{a_0, a_1, a_2, a_3, a_4, a_5, a_6,
 * a_7\} \f$).
 * - kNR: inputs are natural-order and outputs are bit-reversed-order (example of bit-reversed ordering: \f$ \{a_0,
 * a_4, a_2, a_6, a_1, a_5, a_3, a_7\} \f$).
 * - kRN: inputs are bit-reversed-order and outputs are natural-order.
 * - kRR: inputs and outputs are bit-reversed-order.
 */
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ordering {
    kNN,
    kNR,
    kRN,
    kRR,
}

/**
 * @struct NTTConfig
 * Struct that encodes NTT parameters to be passed into the [ntt](@ref ntt) function.
 */
#[repr(C)]
#[derive(Debug)]
pub struct NTTConfig<'a, S> {
    /** Coset generator. Used to perform coset (i)NTTs. Default value: `S::one()` (corresponding to no coset being used). */
    pub coset_gen: S,
    /** Ordering of inputs and outputs. See [Ordering](@ref Ordering). Default value: `Ordering::kNN`. */
    pub ordering: Ordering,
    /** True if inputs are on device and false if they're on host. Default value: false. */
    pub are_inputs_on_device: bool,
    /** If true, output is preserved on device for subsequent use in config and not freed after calculation. Default value: false. */
    pub are_outputs_on_device: bool,
    /** The number of NTTs to compute. Default value: 1. */
    pub batch_size: c_int,
    /** Whether to run the NTT asyncronously. If set to `true`, the NTT function will be non-blocking and you'd need to synchronize
     *  it explicitly by running `cudaStreamSynchronize` or `cudaDeviceSynchronize`. If set to false, the NTT
     *  function will block the current CPU thread. */
    pub is_async: bool,
    /** Details related to the device such as its id and stream id. See [DeviceContext](@ref device_context::DeviceContext). */
    pub ctx: DeviceContext<'a>,
}

// /**
//  * @struct Domain
//  * Struct containing information about the domain on which (i)NTT is evaluated: twiddle factors and coset generator.
//  * Twiddle factors are private, static and can only be set using [GenerateDomain](@ref GenerateDomain) function.
//  * The internal representation of twiddles is prone to change in accordance with changing [NTT](@ref NTT) algorithm.
//  */
// #[repr(C)]
// #[derive(Debug)]
// pub struct Domain<'a, T> {
//     /** Scalar elements that specify a coset to be used in (i)NTT. Default value: None (no coset or alternatively coset
//      *  generated by `S::one()` is used). */
//     pub coset_table: Option<&'a [T]>,
// }
