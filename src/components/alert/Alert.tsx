import { Icon } from "@iconify/react/dist/iconify.js";

interface Props {
  title?: string;
  value: string;
  orientation?: "v" | "h";
}

function Alert({ title, value, orientation = "v" }: Props) {
  return (
    <div
      className={`size-full text-light-green flex ${
        orientation == "v" ? "flex-col" : null
      } justify-center items-center px-1`}
    >
      <Icon icon="material-symbols-light:warning-rounded" className="size-20" />
      <div
        className={`flex flex-col  gap-1 ${
          orientation == "v"
            ? "text-center justify-center items-center"
            : "text-left"
        }`}
      >
        <div className="text-xl  font-bold">{title}</div>
        <div className=" text-sm font-semibold">{value}</div>
      </div>
    </div>
  );
}

export default Alert;
