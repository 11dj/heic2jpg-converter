import { useState, useRef, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { open } from "@tauri-apps/plugin-dialog";
import {
  ImageIcon,
  Upload,
  X,
  FileImage,
  Package,
  CheckCircle,
  AlertCircle,
  Loader2,
  Trash2,
  HardDrive,
  TrendingDown,
  TrendingUp,
} from "lucide-react";
import "./App.css";

interface HeicFileInfo {
  path: string;
  name: string;
  width: number;
  height: number;
  size_bytes: number;
  thumbnail?: string;
}

interface SizeEstimate {
  original_total: number;
  estimated_total: number;
  savings_percent: number;
}

interface ConversionResult {
  success: boolean;
  output_path: string;
  message: string;
}

function formatFileSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
}

function App() {
  const [files, setFiles] = useState<HeicFileInfo[]>([]);
  const [quality, setQuality] = useState<number>(85);
  const [isConverting, setIsConverting] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const [sizeEstimate, setSizeEstimate] = useState<SizeEstimate | null>(null);
  const [status, setStatus] = useState<{
    type: "success" | "error" | "info" | null;
    message: string;
  }>({ type: null, message: "" });
  const fileInputRef = useRef<HTMLInputElement>(null);

  // Listen for Tauri drag-drop events
  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await listen<{ paths: string[] }>("tauri://drag-drop", (event) => {
        if (event.payload.paths && event.payload.paths.length > 0) {
          scanFiles(event.payload.paths);
        }
      });
    };

    setupListener();

    return () => {
      unlisten?.();
    };
  }, []);

  // Calculate size estimate when files or quality changes
  useEffect(() => {
    if (files.length > 0) {
      calculateSizeEstimate();
    } else {
      setSizeEstimate(null);
    }
  }, [files, quality]);

  const calculateSizeEstimate = async () => {
    if (files.length === 0) return;
    
    try {
      const estimate: SizeEstimate = await invoke("calculate_size_estimate", {
        files,
        quality,
      });
      setSizeEstimate(estimate);
    } catch (error) {
      console.error("Failed to calculate size estimate:", error);
    }
  };

  // Handle drag events for visual feedback
  const handleDragEnter = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(true);
  }, []);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
  }, []);

  // Handle HTML5 drop (fallback for browser)
  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setIsDragging(false);

    const items = e.dataTransfer.items;
    if (items.length > 0) {
      await processDroppedItems(items);
    }
  }, []);

  // Process dropped items
  const processDroppedItems = async (items: DataTransferItemList) => {
    const paths: string[] = [];

    for (let i = 0; i < items.length; i++) {
      const item = items[i];
      const entry = item.webkitGetAsEntry?.() || null;

      if (entry) {
        const file = item.getAsFile();
        if (file) {
          const path = (file as any).path;
          if (path) {
            paths.push(path);
          }
        }
      }
    }

    if (paths.length > 0) {
      await scanFiles(paths);
    }
  };

  // Scan HEIC files from paths
  const scanFiles = async (paths: string[]) => {
    try {
      setStatus({ type: "info", message: "Scanning files..." });
      const scannedFiles: HeicFileInfo[] = await invoke("scan_heic_files", {
        paths,
      });
      
      setFiles((prev) => {
        const existingPaths = new Set(prev.map((f) => f.path));
        const newFiles = scannedFiles.filter((f) => !existingPaths.has(f.path));
        return [...prev, ...newFiles];
      });
      
      setStatus({
        type: "success",
        message: `Found ${scannedFiles.length} HEIC file(s)`,
      });
      
      setTimeout(() => setStatus({ type: null, message: "" }), 3000);
    } catch (error) {
      setStatus({
        type: "error",
        message: `Error scanning files: ${error}`,
      });
    }
  };

  // Handle file input change
  const handleFileInput = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const selectedFiles = e.target.files;
    if (!selectedFiles) return;

    const paths: string[] = [];
    for (let i = 0; i < selectedFiles.length; i++) {
      const file = selectedFiles[i];
      const path = (file as any).path;
      if (path) {
        paths.push(path);
      }
    }

    if (paths.length > 0) {
      await scanFiles(paths);
    }

    // Reset input
    e.target.value = "";
  };

  // Remove file from list
  const removeFile = (index: number) => {
    setFiles((prev) => prev.filter((_, i) => i !== index));
  };

  // Clear all files
  const clearAll = () => {
    setFiles([]);
    setSizeEstimate(null);
    setStatus({ type: null, message: "" });
  };

  // Convert and export
  const handleConvert = async () => {
    if (files.length === 0) return;

    try {
      // Ask user for save location
      const savePath = await open({
        directory: true,
        multiple: false,
        title: "Select Output Folder",
      });

      if (!savePath) return;

      setIsConverting(true);
      setStatus({ type: "info", message: "Converting..." });

      const filePaths = files.map((f) => f.path);
      const result: ConversionResult = await invoke("convert_and_export", {
        files: filePaths,
        quality,
        outputDir: savePath,
      });

      if (result.success) {
        setStatus({
          type: "success",
          message: `${result.message} - Saved to: ${result.output_path}`,
        });
      } else {
        setStatus({
          type: "error",
          message: result.message,
        });
      }
    } catch (error) {
      setStatus({
        type: "error",
        message: `Conversion failed: ${error}`,
      });
    } finally {
      setIsConverting(false);
    }
  };

  return (
    <div id="root">
      <header className="header">
        <h1>
          <ImageIcon className="header-icon" />
          HEIC2JPG Converter
        </h1>
      </header>

      <main className="main-container">
        {/* Drop Zone */}
        <div
          className={`drop-zone ${isDragging ? "drag-over" : ""}`}
          onDragEnter={handleDragEnter}
          onDragLeave={handleDragLeave}
          onDragOver={handleDragOver}
          onDrop={handleDrop}
          onClick={() => fileInputRef.current?.click()}
        >
          <input
            ref={fileInputRef}
            type="file"
            style={{ display: "none" }}
            onChange={handleFileInput}
            // @ts-ignore - webkitdirectory allows folder selection
            webkitdirectory=""
            directory=""
            multiple
            accept=".heic,.heif,.zip"
          />
          <Upload className="drop-zone-icon" />
          <h3>Drop HEIC files here</h3>
          <p>
            Support: single file, multiple files, folders, or ZIP archives
          </p>
        </div>

        {/* Quality Control */}
        <div className="controls-section">
          <div className="quality-control">
            <label>JPEG Quality</label>
            <input
              type="range"
              min="1"
              max="100"
              value={quality}
              onChange={(e) => setQuality(Number(e.target.value))}
              className="quality-slider"
            />
            <span className="quality-value">{quality}%</span>
          </div>
          
          {/* Size Estimate */}
          {sizeEstimate && (
            <div className="size-estimate">
              <div className="estimate-row">
                <span className="estimate-label">
                  <HardDrive size={14} />
                  Original:
                </span>
                <span className="estimate-value original">
                  {formatFileSize(sizeEstimate.original_total)}
                </span>
              </div>
              <div className="estimate-row">
                <span className="estimate-label">
                  <Package size={14} />
                  Estimated:
                </span>
                <span className={`estimate-value ${sizeEstimate.savings_percent > 0 ? 'smaller' : 'larger'}`}>
                  {formatFileSize(sizeEstimate.estimated_total)}
                </span>
              </div>
              <div className="estimate-row">
                <span className="estimate-label">
                  {sizeEstimate.savings_percent > 0 ? (
                    <TrendingDown size={14} className="savings-down" />
                  ) : (
                    <TrendingUp size={14} className="savings-up" />
                  )}
                  {sizeEstimate.savings_percent > 0 ? 'Savings:' : 'Increase:'}
                </span>
                <span className={`estimate-value ${sizeEstimate.savings_percent > 0 ? 'savings' : 'increase'}`}>
                  {Math.abs(sizeEstimate.savings_percent).toFixed(1)}%
                </span>
              </div>
            </div>
          )}
        </div>

        {/* File List */}
        <div className="file-list-section">
          <div className="file-list-header">
            <h3>Files to Convert</h3>
            {files.length > 0 && (
              <span className="file-count">{files.length}</span>
            )}
          </div>

          {files.length === 0 ? (
            <div className="file-list-empty">
              <FileImage />
              <p>No files selected</p>
            </div>
          ) : (
            <div className="file-list">
              {files.map((file, index) => (
                <div key={file.path} className="file-item">
                  {file.thumbnail ? (
                    <img
                      src={file.thumbnail}
                      alt={file.name}
                      className="file-thumbnail"
                    />
                  ) : (
                    <div className="file-thumbnail-placeholder">
                      <ImageIcon size={20} />
                    </div>
                  )}
                  <div className="file-info">
                    <div className="file-name">{file.name}</div>
                    <div className="file-meta">
                      {file.width}×{file.height} • {formatFileSize(file.size_bytes)}
                    </div>
                  </div>
                  <button
                    className="file-remove"
                    onClick={() => removeFile(index)}
                    title="Remove"
                  >
                    <X size={16} />
                  </button>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Status Message */}
        {status.type && (
          <div className={`status-bar ${status.type}`}>
            {status.type === "info" && isConverting ? (
              <Loader2 size={16} className="spinner" />
            ) : status.type === "success" ? (
              <CheckCircle size={16} />
            ) : status.type === "error" ? (
              <AlertCircle size={16} />
            ) : (
              <Loader2 size={16} className="spinner" />
            )}
            <span>{status.message}</span>
          </div>
        )}

        {/* Footer Actions */}
        <div className="footer">
          <button
            className="clear-btn"
            onClick={clearAll}
            disabled={files.length === 0 || isConverting}
            title="Clear all"
          >
            <Trash2 size={20} />
          </button>
          <button
            className={`convert-btn ${isConverting ? "converting" : ""}`}
            onClick={handleConvert}
            disabled={files.length === 0 || isConverting}
          >
            {isConverting ? (
              <>
                <Loader2 size={20} className="spinner" />
                Converting...
              </>
            ) : (
              <>
                <Package size={20} />
                Convert to JPG & Export ZIP
              </>
            )}
          </button>
        </div>
      </main>
    </div>
  );
}

export default App;
