#include <cuda_runtime.h>
#include "../../include/types.h"
#include <stdbool.h>

#ifndef _BN254_NTT_H
#define _BN254_NTT_H

#ifdef __cplusplus
extern "C" {
#endif

cudaError_t bn254NTTCuda(const scalar_t* input, int size, int dir, NTTConfig* config, scalar_t* output);
cudaError_t bn254ECNTTCuda(const projective_t* input, int size, int dir, NTTConfig* config, projective_t* output);
cudaError_t bn254InitializeDomain(scalar_t* primitive_root, DeviceContext* ctx, bool fast_twiddles);
cudaError_t bn254ReleaseDomain(DeviceContext* ctx);

#ifdef __cplusplus
}
#endif

#endif