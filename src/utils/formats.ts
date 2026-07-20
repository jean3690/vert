import type { FileFormat } from '../types';

export const FORMAT_EXTENSIONS: Record<FileFormat, string[]> = {
  properties: ['.properties'],
  yaml: ['.yaml', '.yml'],
  json: ['.json'],
  toml: ['.toml'],
  xml: ['.xml'],
  ini: ['.ini', '.cfg', '.conf'],
  csv: ['.csv'],
  epub: ['.epub'],
  docx: ['.docx'],
  pdf: ['.pdf'],
  markdown: ['.md', '.markdown'],
  html: ['.html', '.htm'],
  txt: ['.txt', '.text'],
  svg: ['.svg'],
  png: ['.png'],
  jpeg: ['.jpg', '.jpeg', '.jpe'],
  gif: ['.gif'],
  webp: ['.webp'],
  bmp: ['.bmp', '.dib'],
  tiff: ['.tiff', '.tif'],
};

export const FORMAT_LABELS: Record<FileFormat, string> = {
  properties: 'Properties',
  yaml: 'YAML',
  json: 'JSON',
  toml: 'TOML',
  xml: 'XML',
  ini: 'INI',
  csv: 'CSV',
  epub: 'EPUB',
  docx: 'Word (DOCX)',
  pdf: 'PDF',
  markdown: 'Markdown',
  html: 'HTML',
  txt: 'Plain Text',
  svg: 'SVG',
  png: 'PNG',
  jpeg: 'JPEG',
  gif: 'GIF',
  webp: 'WebP',
  bmp: 'BMP',
  tiff: 'TIFF',
};

export const FORMAT_CATEGORY: Record<FileFormat, 'config' | 'data' | 'document' | 'image'> = {
  properties: 'config',
  yaml: 'config',
  json: 'config',
  toml: 'config',
  xml: 'config',
  ini: 'config',
  csv: 'data',
  epub: 'document',
  docx: 'document',
  pdf: 'document',
  markdown: 'document',
  html: 'document',
  txt: 'document',
  svg: 'image',
  png: 'image',
  jpeg: 'image',
  gif: 'image',
  webp: 'image',
  bmp: 'image',
  tiff: 'image',
};

// Must stay in sync with Rust Format::valid_targets()
export const VALID_CONVERSIONS: Record<FileFormat, FileFormat[]> = {
  properties: ['yaml', 'json', 'toml', 'xml', 'ini'],
  yaml: ['properties', 'json', 'toml', 'xml', 'ini'],
  json: ['properties', 'yaml', 'toml', 'xml', 'ini', 'csv'],
  toml: ['properties', 'yaml', 'json', 'xml', 'ini'],
  xml: ['properties', 'yaml', 'json', 'toml', 'ini', 'csv'],
  ini: ['properties', 'yaml', 'json', 'toml', 'xml'],
  csv: ['json', 'xml'],
  epub: ['pdf', 'html', 'markdown', 'txt'],
  docx: ['pdf', 'html', 'markdown', 'txt'],
  pdf: ['txt'],
  markdown: ['html', 'pdf', 'txt'],
  html: ['pdf', 'markdown', 'txt'],
  txt: ['markdown', 'html', 'pdf'],
  svg: ['png', 'jpeg', 'gif', 'webp', 'bmp', 'tiff', 'txt'],
  png: ['jpeg', 'gif', 'webp', 'bmp', 'tiff'],
  jpeg: ['png', 'gif', 'webp', 'bmp', 'tiff'],
  gif: ['png', 'jpeg', 'webp', 'bmp', 'tiff'],
  webp: ['png', 'jpeg', 'gif', 'bmp', 'tiff'],
  bmp: ['png', 'jpeg', 'gif', 'webp', 'tiff'],
  tiff: ['png', 'jpeg', 'gif', 'webp', 'bmp'],
};

export function detectFormat(fileName: string): FileFormat | null {
  const lower = fileName.toLowerCase();
  for (const [format, extensions] of Object.entries(FORMAT_EXTENSIONS)) {
    for (const ext of extensions) {
      if (lower.endsWith(ext)) {
        return format as FileFormat;
      }
    }
  }
  return null;
}

export function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
}
