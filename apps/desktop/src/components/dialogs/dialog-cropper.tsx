import { Dialog, DialogContent, DialogHeader, DialogTitle } from "../ui/dialog";
import {
  ImageCrop,
  ImageCropApply,
  ImageCropContent,
  ImageCropReset,
  type CropResult
} from "@/components/kibo-ui/image-crop";
import { Button } from "../ui/button";

type Props = {
  isOpen?: boolean;
  previewUrl: string | null;
  onClose: () => void;
  onSuccess: (crop: CropResult) => void;
};

export function DialogCropper({
  isOpen: controlledOpen,
  previewUrl,
  onClose,
  onSuccess
}: Props) {
  const isOpen = controlledOpen ?? !!previewUrl;

  return (
    <Dialog open={isOpen} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Crop Image</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col items-center justify-center py-4">
          {previewUrl && (
            <ImageCrop
              aspect={1}
              imageUrl={previewUrl}
              onCrop={onSuccess}
              circularCrop
            >
              <ImageCropContent className="max-w-md " />
              <div className="flex items-center justify-end gap-2 mt-4">
                <ImageCropReset asChild>
                  <Button size="sm" variant="ghost">
                    Reset Zoom
                  </Button>
                </ImageCropReset>

                <Button onClick={onClose} size="sm" variant="ghost">
                  Cancel
                </Button>

                <ImageCropApply asChild>
                  <Button size="sm">Apply Crop</Button>
                </ImageCropApply>
              </div>
            </ImageCrop>
          )}
        </div>
      </DialogContent>
    </Dialog>
  );
}
