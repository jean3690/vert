/** All supported file formats */
export type FileFormat =
  | 'properties' | 'yaml' | 'json' | 'toml' | 'xml'
  | 'csv'
  | 'docx' | 'pdf'
  | 'markdown' | 'html';

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
