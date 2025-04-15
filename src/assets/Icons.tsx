import React, { ComponentProps, ComponentPropsWithoutRef } from "react";

interface CopyIconProps extends ComponentPropsWithoutRef<"svg"> {
  strokeColor?: string;
  size?: number;
}

const CopyIcon: React.FC<CopyIconProps> = ({
  strokeColor = "currentColor",
  size = 24,
  ...svgProps
}) => (
  <svg
    {...svgProps}
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    fill="none"
    stroke={strokeColor}
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className={"lucide lucide-copy cursor-pointer"}
  >
    <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
    <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
  </svg>
);

interface DeleteIconProps extends ComponentPropsWithoutRef<"svg"> {
  strokeColor?: string;
  size?: number;
}

const DeleteIcon: React.FC<DeleteIconProps> = ({
  strokeColor = "currentColor",
  size = 24,
  ...svgProps
}) => (
  <svg
    {...svgProps}
    xmlns="http://www.w3.org/2000/svg"
    width={size}
    height={size}
    fill="none"
    stroke={strokeColor}
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
    className={"lucide lucide-trash cursor-pointer"}
  >
    <polyline points="3 6 5 6 21 6" />
    <path d="M19 6L17.5 20a2 2 0 0 1-2 2H8.5a2 2 0 0 1-2-2L5 6" />
    <path d="M10 11v6" />
    <path d="M14 11v6" />
  </svg>
);

export { CopyIcon, DeleteIcon };
