import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "FlowFi | Real-time Payment Streams",
  description: "The trustless infrastructure to stream salaries, tokens, and rewards in real-time.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className="antialiased">
        {children}
      </body>
    </html>
  );
}
