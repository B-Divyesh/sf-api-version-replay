export {};

declare global {
  interface Window {
    __storageReads: string[];
    __storageWrites: string[];
  }
}
