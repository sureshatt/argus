import React from "react";

interface Props {
  children: React.ReactNode;
}

function CardHeader({ children }: Props) {
  return <div className="h-14 flex items-center px-3 ">{children}</div>;
}

export default CardHeader;
