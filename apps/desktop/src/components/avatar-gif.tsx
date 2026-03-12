import { useEffect, useState } from "react";
import { AvatarImage } from "./ui/avatar";

type Props = {
  src: string;
  alt: string;
  className?: string;
  animated?: boolean;
};

export function AvatarGif({ src, alt, className, animated }: Props) {
  const [staticSrc, setStaticSrc] = useState<string | null>(null);

  const isGif = src?.endsWith(".gif") || src?.includes(".gif?");

  useEffect(() => {
    if (!isGif) return;
    const img = new Image();
    img.crossOrigin = "anonymous";
    img.onload = () => {
      const canvas = document.createElement("canvas");
      canvas.width = img.width;
      canvas.height = img.height;
      canvas.getContext("2d")?.drawImage(img, 0, 0);
      setStaticSrc(canvas.toDataURL("image/png"));
    };
    img.src = src;
  }, [src, isGif]);

  if (!isGif || !staticSrc) {
    return <AvatarImage src={src} alt={alt} className={className} />;
  }

  return (
    <AvatarImage
      src={animated ? src : staticSrc}
      alt={alt}
      className={className}
      onMouseEnter={(e) => (e.currentTarget.src = src)}
      onMouseLeave={(e) => (e.currentTarget.src = staticSrc)}
    />
  );
}
