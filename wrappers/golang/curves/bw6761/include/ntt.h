#include <cuda_runtime.h>
#include "../../include/types.h"
#include <stdbool.h>

#ifndef _BW6_761_NTT_H
#define _BW6_761_NTT_H

#ifdef __cplusplus
extern "C" {
#endif

cudaError_t bw6_761NTTCuda(const scalar_t* input, int size, int dir, NTTConfig* config, scalar_t* output);
cudaError_t bw6_761ECNTTCuda(const projective_t* input, int size, int dir, NTTConfig* config, projective_t* output);
cudaError_t bw6_761InitializeDomain(scalar_t* primitive_root, DeviceContext* ctx, bool fast_twiddles);
cudaError_t bw6_761ReleaseDomain(DeviceContext* ctx);

#ifdef __cplusplus
}
#endif

#endif