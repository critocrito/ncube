import React from "react";

interface CardProps {
  children: React.ReactNode;
}

const Card = ({children}: CardProps) => {
  return (
    <div className="h-40 bg-white p-8 shadow-md flex items-center">
      {children}
    </div>
  );
};

export default Card;
