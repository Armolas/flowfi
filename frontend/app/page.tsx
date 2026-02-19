import { Navbar } from "@/components/Navbar";
import { Hero } from "@/components/Hero";
import { Stats } from "@/components/Stats";
import { Features } from "@/components/Features";
import { HowItWorks } from "@/components/HowItWorks";
import { FAQ } from "@/components/FAQ";
import { Footer } from "@/components/Footer";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col bg-background font-sans text-foreground">
      {/* Dynamic Background Glows */}
      <div className="pointer-events-none fixed inset-0 z-0 overflow-hidden">
        <div className="absolute -top-[10%] -left-[10%] h-[50%] w-[50%] animate-pulse-slow rounded-full bg-accent opacity-10 blur-[140px]"></div>
        <div className="absolute top-[30%] -right-[10%] h-[40%] w-[40%] animate-pulse-slow rounded-full bg-accent-secondary opacity-5 blur-[120px] [animation-delay:2s]"></div>
        <div className="absolute -bottom-[10%] left-[20%] h-[30%] w-[30%] animate-pulse-slow rounded-full bg-accent-tertiary opacity-5 blur-[100px] [animation-delay:4s]"></div>
      </div>

      <Navbar />

      <main className="relative z-10 flex-1">
        <Hero />
        <Stats />
        <Features />
        <HowItWorks />

        {/* Call to Action Section (Simplified here or modularized if reused) */}
        <section className="py-40 px-6 text-center relative overflow-hidden">
          <div className="absolute inset-0 bg-accent/5 opacity-50"></div>
          <div className="glass-card mx-auto max-w-5xl rounded-[2.5rem] p-16 md:p-32 relative z-10 border-accent/20">
            <h2 className="text-5xl md:text-7xl font-black mb-8 leading-tight">
              Ready to build the <br />
              <span className="text-gradient">Money Network?</span>
            </h2>
            <p className="mx-auto mt-8 max-w-xl text-xl text-slate-400 font-medium">
              Join the hundreds of protocol builders and DAOs who have already switched to real-time capital allocation.
            </p>
            <div className="mt-14 flex flex-col sm:flex-row justify-center gap-6">
              <button className="h-14 px-10 rounded-full bg-accent text-background font-bold text-lg shadow-[0_0_20px_rgba(16,185,129,0.4)] hover:shadow-[0_0_30px_rgba(16,185,129,0.6)] transition-all active:scale-95">
                Launch App
              </button>
              <button className="h-14 px-10 rounded-full border border-glass-border bg-glass backdrop-blur-xl font-bold text-lg hover:bg-white/5 transition-all active:scale-95">
                Contact Sales
              </button>
            </div>
          </div>
        </section>

        <FAQ />
      </main>

      <Footer />
    </div>
  );
}
