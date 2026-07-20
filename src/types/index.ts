/** All supported file formats */
export type FileFormat =
  | 'properties' | 'yaml' | 'json' | 'toml' | 'xml' | 'ini'
  | 'csv'
  | 'epub' | 'docx' | 'pdf'
  | 'markdown' | 'html'
  | 'txt'
  | 'svg' | 'png' | 'jpeg' | 'gif' | 'webp' | 'bmp' | 'tiff';

/** Result returned by the convert_file Rust command */
export interface ConvertResult {
  outputPath: string;
  outputSize: number;
}

/** A file queued for conversion */
export interface QueuedFile {
  path: string;
  name: string;
  detectedFormat: FileFormat | null;
}

/** Conversion status */
export type ConversionStatus = 'idle' | 'converting' | 'success' | 'error';
