interface Props {
  value: string;
}

function CardTitle({ value }: Props) {
  return (
    <div className="font-quantic font-bold text-xl  text-light-green">
      {value}
    </div>
  );
}

export default CardTitle;
