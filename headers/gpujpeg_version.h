/**
 * @file
 * Copyright (c) 2016-2023, CESNET z.s.p.o
 *
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 *     * Redistributions of source code must retain the above copyright
 *       notice, this list of conditions and the following disclaimer.
 *
 *     * Redistributions in binary form must reproduce the above copyright
 *       notice, this list of conditions and the following disclaimer in the
 *       documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef GPUJPEG_VERSION_H
#define GPUJPEG_VERSION_H

#define GPUJPEG_VERSION_MAJOR 0
#define GPUJPEG_VERSION_MINOR 27
#define GPUJPEG_VERSION_PATCH 2

#define GPUJPEG_MK_VERSION_INT(major, minor, patch) ((major) << 16U | (minor) << 8U | (patch))

#define GPUJPEG_VERSION_INT GPUJPEG_MK_VERSION_INT(GPUJPEG_VERSION_MAJOR, GPUJPEG_VERSION_MINOR, GPUJPEG_VERSION_PATCH)

/// @deprecated use @ref GPUJPEG_VERSION_INT instead
#define LIBGPUJPEG_API_VERSION ((GPUJPEG_VERSION_MAJOR << 8U) | \
                GPUJPEG_VERSION_MINOR)

#endif // GPUJPEG_VERSION_H
