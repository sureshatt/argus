import React from "react";

interface Props {
  children: React.ReactNode;
}

function CardBody({ children }: Props) {
  return (
    <div className="size-full flex flex-col items-center overflow-hidden pb-2">
      <div className="size-full overflow-y-auto custom-scrollbar ">
        {children}
      </div>
    </div>
  );
}

export default CardBody;
