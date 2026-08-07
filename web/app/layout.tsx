import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Penta — Magic Format Simulator",
  description:
    "Play deterministic Old School 93/94 and ISD–RTR Standard Magic against Rust-powered bots.",
  openGraph: {
    title: "Penta",
    description: "Classic formats. Deterministic games. Rust-powered rules.",
    images: ["/og.png"],
  },
  twitter: {
    card: "summary_large_image",
    title: "Penta",
    description: "Classic formats. Deterministic games. Rust-powered rules.",
    images: ["/og.png"],
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
