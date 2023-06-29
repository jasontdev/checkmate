type BlueButtonProps = {
  title: string;
  solid?: boolean;
  onClick: () => void;
};

function Button({ title, solid, onClick }: BlueButtonProps) {
  if (solid) {
    return (
      <button className="rounded bg-red-500 px-2 py-1 font-bold text-white">
        {title}
      </button>
    );
  }

  return (
    <button
      className="rounded px-2 py-1 font-bold text-red-500"
      onClick={() => onClick()}
    >
      {title}
    </button>
  );
}

export default Button;
