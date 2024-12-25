import type { Metadata } from "next";
export const metadata: Metadata = {
  title: "mandolin",
  description: "Generate openapi-based server code",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>
        {children}
      </body>
    </html>
  );
}
