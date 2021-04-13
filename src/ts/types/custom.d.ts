declare module "*.svg" {
  const content: string;
  export default content;
}

declare module "*.mdx" {
  let MDXComponent: (props: any) => JSX.Element;
  export default MDXComponent;
}
