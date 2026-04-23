import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { cva, type VariantProps } from "class-variance-authority";

import { cn } from "@/lib/utils";

const buttonVariants = cva(
  "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-full text-sm font-medium transition-all duration-200 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default:
          "bg-primary px-5 py-2.5 text-primary-foreground shadow-[0_10px_24px_rgba(0,129,167,0.18)] hover:-translate-y-0.5 hover:opacity-95",
        secondary:
          "bg-secondary px-5 py-2.5 text-secondary-foreground hover:-translate-y-0.5 hover:bg-secondary/85",
        outline:
          "border bg-white/70 px-5 py-2.5 hover:-translate-y-0.5 hover:bg-muted/80",
        ghost: "px-4 py-2 text-foreground hover:bg-muted/75"
      },
      size: {
        default: "",
        lg: "px-6 py-3 text-base",
        sm: "px-3.5 py-2 text-xs",
        icon: "size-10 rounded-full"
      }
    },
    defaultVariants: {
      variant: "default",
      size: "default"
    }
  }
);

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement>,
    VariantProps<typeof buttonVariants> {
  asChild?: boolean;
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant, size, asChild = false, ...props }, ref) => {
    const Comp = asChild ? Slot : "button";
    return <Comp className={cn(buttonVariants({ variant, size, className }))} ref={ref} {...props} />;
  }
);
Button.displayName = "Button";

export { Button, buttonVariants };
