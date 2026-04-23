import * as React from "react";

import { cn } from "@/lib/utils";

export const Textarea = React.forwardRef<
  HTMLTextAreaElement,
  React.TextareaHTMLAttributes<HTMLTextAreaElement>
>(({ className, ...props }, ref) => {
  return (
    <textarea
      ref={ref}
      className={cn(
        "flex min-h-[120px] w-full rounded-[1.2rem] border bg-white/88 px-4 py-3 text-sm outline-none transition placeholder:text-muted-foreground focus-visible:ring-4 focus-visible:ring-primary/10",
        className
      )}
      {...props}
    />
  );
});

Textarea.displayName = "Textarea";
