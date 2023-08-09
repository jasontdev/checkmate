import React from "react";

type ContainerProps = {
  children: React.ReactNode;
}
function Container({children}: ContainerProps) {
  return <div className="w-full h-full">{children}</div>
}

export default Container;