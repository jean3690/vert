import type { FileFormat } from '../types';

export const FORMAT_EXTENSIONS: Record<FileFormat, string[]> = {
  properties: ['.properties'],
  yaml: ['.yaml', '.yml'],
  json: ['.json'],
  toml: ['.toml'],
  xml: ['.xml'],
  csv: ['.csv'],
  docx: ['.docx'],
  pdf: ['.pdf'],
  markdown: ['.md', '.markdown'],
  html: ['.html', '.htm'],
};

export const FORMAT_LABELS: Record<FileFormat, string> = {
  properties: 'Properties',
  yaml: 'YAML',
  json: 'JSON',
  toml: 'TOML',
  xml: 'XML',
  csv: 'CSV',
  docx: 'Word (DOCX)',
  pdf: 'PDF',
  markdown: 'Markdown',
  html: 'HTML',
};

export const FORMAT_CATEGORY: Record<FileFormat, 'config' | 'data' | 'document'> = {
  properties: 'config',
  yaml: 'config',
  json: 'config',
  toml: 'config',
  xml: 'config',
  csv: 'data',
  docx: 'document',
  pdf: 'document',
  markdown: 'document',
  html: 'document',
};

// Must stay in sync with Rust Format::valid_targets()
export const VALID_CONVERSIONS: Record<FileFormat, FileFormat[]> = {
  properties: ['yaml', 'json', 'toml', 'xml'],
  yaml: ['properties', 'json', 'toml', 'xml'],
  json: ['properties', 'yaml', 'toml', 'xml', 'csv'],
  toml: ['properties', 'yaml', 'json', 'xml'],
  xml: ['properties', 'yaml', 'json', 'toml', 'csv'],
  csv: ['json', 'xml'],
  docx: ['pdf', 'html', 'markdown'],
  pdf: [],
  markdown: ['html', 'pdf'],
  html: ['pdf', 'markdown'],
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
