export const unreachable = (message?: string): never => {
  if (message === undefined) {
    throw new Error("Unreachable code reached.");
  } else {
    throw new Error(`Unreachable code reached: ${message}`);
  }
};

export const isString = (x: unknown): x is string => {
  return typeof x === "string";
};
