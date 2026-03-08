import { useState } from "react";
import { open as openDialog } from "@tauri-apps/plugin-dialog";
import { toast } from "sonner";
import { readFile } from "@tauri-apps/plugin-fs";

export function useImageSelection() {
  const [originalPath, setOriginalPath] = useState<string | null>(null);

  const [previewUrl, setPreviewUrl] = useState<string | null>(null);
  const [isSelecting, setIsSelecting] = useState(false);

  const handleSelectImage = async () => {
    setIsSelecting(true);
    try {
      const selected = await openDialog({
        multiple: false,
        filters: [
          { name: "Image", extensions: ["png", "jpg", "jpeg", "gif", "webp"] }
        ]
      });

      if (selected && typeof selected === "string") {
        setOriginalPath(selected);

        const contents = await readFile(selected);
        const ext = selected.split(".").pop()?.toLowerCase();
        const mimeMap: Record<string, string> = {
          png: "image/png",
          jpg: "image/jpeg",
          jpeg: "image/jpeg",
          gif: "image/gif",
          webp: "image/webp"
        };
        const mime = mimeMap[ext ?? ""] ?? "image/png";
        const blob = new Blob([contents], { type: mime });
        const blobUrl = URL.createObjectURL(blob);
        setPreviewUrl(blobUrl);
      }
    } catch (err) {
      console.error(err);
      toast.error("Could not open file dialog");
    } finally {
      setIsSelecting(false);
    }
  };

  const clearSelection = () => {
    setOriginalPath(null);
    setPreviewUrl(null);
  };

  return {
    originalPath,
    previewUrl,
    handleSelectImage,
    clearSelection,
    isSelecting
  };
}
