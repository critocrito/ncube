import React from "react";

type DescriptionItem = {
  label: string;
  key?: string;
  value?: string | JSX.Element;
};

interface DescriptionProps {
  items: DescriptionItem[];
}

const Description = ({items}: DescriptionProps) => {
  return (
    <div className="border-t border-solitude py-5">
      <dl className="grid grid-cols-1 gap-x-4 gap-y-6 sm:grid-cols-2">
        {items
          .filter(
            ({value}) => value !== null && value !== undefined && value !== "",
          )
          .map(({label, value, key}) => {
            return (
              <div key={key || label} className="sm:col-span-2">
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
