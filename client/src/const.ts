export const AUDIO_EXTENSIONS = [
  ".mp3",
  ".wav",
  ".ogg",
  ".m4a",
  ".flac",
  ".aac",
  ".wma",
  ".opus",
  ".mp2",
  ".wv",
  ".mka",
  ".tta",
  ".aiff",
  ".dsf",
  ".dsd",
  ".dff",
] as const;
export function isAudioFile(filename: string) {
  return AUDIO_EXTENSIONS.some((ext) => filename.endsWith(ext));
}

export const IMAGE_EXTENSIONS = [
  ".jpg",
  ".jpeg",
  ".png",
  ".gif",
  ".bmp",
  ".tiff",
  ".tif",
  ".webp",
  ".svg",
  ".ico",
] as const;
export function isImageFile(filename: string) {
  return IMAGE_EXTENSIONS.some((ext) => filename.endsWith(ext));
}

export const SERVER_HOST = import.meta.env.DEV
  ? "localhost:14567"
  : location.host;
