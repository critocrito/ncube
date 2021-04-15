import React from "react";

type DescriptionItem = {label: string; value?: string};

interface DescriptionProps {
  items: DescriptionItem[];
}

const Description = ({items}: DescriptionProps) => {
  return (
    <div className="border-t border-solitude py-5">
      <dl className="grid grid-cols-1 gap-x-4 gap-y-8 sm:grid-cols-2">
        {items
          .filter(
            ({value}) => value !== null && value !== undefined && value !== "",
          )
          .map(({label, value}) => {
            return (
              <div key={label} className="sm:col-span-2">
                <dt className="font-bold text-sapphire">{label}</dt>
                <dd className="mt-1">{value}</dd>
              </div>
            );
          })}
      </dl>
    </div>
  );
};

export default Description;
