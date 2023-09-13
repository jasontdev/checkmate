import React from "react";
import {Outlet} from "react-router-dom";

function App() {
  return (<div className="h-screen w-screen">
    <Outlet />
  </div>);
}

export {App};
