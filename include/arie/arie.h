#ifndef ARIE_LIB_AUDIO_H
#define ARIE_LIB_AUDIO_H

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

void* AudioPlayerCreate(uint32_t sampleRate, uint16_t channels);
void AudioPlayerFree(void* player);

size_t AudioPlayerGetBufferredSampleCount(void* player);
size_t AudioPlayerGetBufferSize(void* player);

void AudioPlayerPlay(void* player);
void AudioPlayerPause(void* player);

void AudioPlayerQueueBuffer(void* player, const uint8_t* buf, size_t len);

#ifdef __cplusplus
}
#endif

#endif /* ARIE_LIB_AUDIO_H */
