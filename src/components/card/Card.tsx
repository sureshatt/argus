import { cn } from "../../utils/tools";

interface Props {
  cls?: string;
  children?: React.ReactNode;
}

function Card({ cls = "", children }: Props) {
  return (
    <div
      className={cn(
        "relative flex flex-col bg-[#011510]/70 h-96 w-96 opacity-100 rounded-[20px] backdrop-blur-xs shadow-card-inset border-1 border-cyan-900",
        cls
      )}
    >
      {children}
    </div>
  );
}

export default Card;
