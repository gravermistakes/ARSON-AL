/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState } from 'react';
import { 
  Shield, 
  Terminal, 
  Activity, 
  FileText, 
  Settings, 
  ChevronRight,
  Zap,
  Lock,
  Cpu
} from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@/src/lib/utils';

interface NavItemProps {
  icon: React.ElementType;
  label: string;
  active: boolean;
  onClick: () => void;
}

const NavItem = ({ icon: Icon, label, active, onClick }: NavItemProps) => (
  <button
    onClick={onClick}
    className={cn(
      "w-full flex items-center space-x-3 px-4 py-3 text-[10px] font-mono uppercase tracking-[0.2em] transition-all duration-200 group relative",
      active 
        ? "text-accent bg-accent/5 border-l-2 border-accent" 
        : "text-zinc-500 hover:text-zinc-300 hover:bg-zinc-900/50"
    )}
  >
    <Icon className={cn("w-4 h-4", active ? "text-accent" : "text-zinc-500 group-hover:text-zinc-300")} />
    <span>{label}</span>
  </button>
);

export const AetheriaLayout: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [activeTab, setActiveTab] = useState('surfaces');

  return (
    <div className="flex h-screen w-full bg-void overflow-hidden text-zinc-400 font-sans p-4 space-y-4 flex-col">
      {/* Header */}
      <header className="flex items-center justify-between border-b border-edge pb-4 px-2 shrink-0">
        <div className="flex items-center space-x-4">
          <div className="w-8 h-8 bg-accent rounded-sm flex items-center justify-center font-bold text-black text-xs font-mono">GP</div>
          <div>
            <h1 className="text-lg font-semibold tracking-tighter uppercase text-white">GeminiProtocol <span className="text-accent opacity-70">v4.2.0-Alpha</span></h1>
            <p className="text-[10px] font-mono uppercase tracking-widest text-gray-500">Operational Adversarial Environment // Active Instance</p>
          </div>
        </div>
        <div className="flex space-x-8 items-center font-mono text-[11px]">
          <div className="flex items-center space-x-2">
            <span className="w-1.5 h-1.5 rounded-full bg-green-500 shadow-[0_0_8px_#22c55e]"></span>
            <span className="text-gray-400">ENCRYPTED CHANNEL</span>
          </div>
          <div className="flex items-center space-x-3 text-gray-400">
            <span>LATENCY: 14MS</span>
            <span className="px-2 py-0.5 bg-zinc-800 text-zinc-300 rounded-sm">AES-256</span>
          </div>
        </div>
      </header>

      <div className="flex-1 flex space-x-4 min-h-0">
        {/* Sidebar */}
        <aside className="w-64 flex flex-col space-y-4 shrink-0 overflow-y-auto pr-1">
          <section className="border border-edge bg-panel p-4 flex flex-col space-y-4 flex-1">
            <h2 className="text-[10px] font-bold text-accent uppercase tracking-widest mb-2">Mixture of Experts</h2>
            <div className="space-y-4 flex-1">
              <div className="p-3 border-l-2 border-accent bg-zinc-900/40">
                <div className="flex justify-between items-start">
                  <span className="text-[11px] font-bold text-white font-mono">MICRO_PT_BETA</span>
                  <span className="text-[9px] bg-cyan-900/50 text-accent px-1 border border-accent/30 font-mono">ACTIVE</span>
                </div>
                <p className="text-[10px] text-zinc-500 mt-1 font-mono uppercase">Penetration Specialist</p>
                <div className="w-full bg-zinc-800 h-1 mt-3">
                  <motion.div initial={{ width: 0 }} animate={{ width: '75%' }} className="bg-accent h-full shadow-[0_0_8px_#00F0FF]" />
                </div>
              </div>
              <div className="p-3 border-l-2 border-indigo-400 bg-zinc-900/40 opacity-70">
                <div className="flex justify-between items-start">
                  <span className="text-[11px] font-bold text-white font-mono">MICRO_PROG_V3</span>
                  <span className="text-[9px] bg-indigo-900/50 text-indigo-300 px-1 border border-indigo-500/30 font-mono">IDLE</span>
                </div>
                <p className="text-[10px] text-zinc-500 mt-1 font-mono uppercase">Programming Expert</p>
              </div>
            </div>

            <div className="pt-4 border-t border-edge">
              <h3 className="text-[10px] font-bold text-zinc-500 uppercase tracking-widest mb-3">Attention Cycle</h3>
              <div className="grid grid-cols-4 gap-1">
                {[0.1, 0.4, 0.8, 0.1].map((op, i) => (
                  <motion.div
                    key={i}
                    animate={i === 2 ? { opacity: [0.3, 0.8, 0.3] } : {}}
                    transition={{ duration: 1.5, repeat: Infinity }}
                    style={{ opacity: op }}
                    className="h-6 bg-accent"
                  />
                ))}
              </div>
            </div>
          </section>

          <section className="border border-edge bg-panel p-4 h-32 flex flex-col">
            <h2 className="text-[10px] font-bold text-accent uppercase tracking-widest mb-3">Database</h2>
            <div className="flex-1 flex items-center justify-center border-2 border-dashed border-edge h-full rounded-sm cursor-pointer hover:bg-white/5 transition-colors group">
              <div className="flex flex-col items-center space-y-1">
                <Database className="w-4 h-4 text-zinc-700 group-hover:text-accent transition-colors" />
                <span className="text-[10px] text-zinc-600 font-mono group-hover:text-zinc-400 uppercase tracking-tighter">Upload Target DB</span>
              </div>
            </div>
          </section>
        </aside>

        {/* Main Content Area */}
        <main className="flex-1 flex flex-col space-y-4 min-w-0">
          <section className="flex-1 bg-panel border border-edge flex flex-col relative min-h-0 overflow-y-auto">
             {children}
          </section>
        </main>
      </div>

      {/* Footer */}
      <footer className="shrink-0 flex items-center justify-between border-t border-edge pt-4 px-2">
        <div className="flex space-x-8 text-[10px] font-mono text-zinc-600">
          <div className="flex items-center space-x-2">
            <span className="text-white">MODE:</span> 
            <span className="text-accent underline decoration-accent/30 underline-offset-4 tracking-[0.2em]">ADVERSARIAL_AUTONOMOUS</span>
          </div>
          <div className="flex items-center space-x-2">
            <span className="text-white">UPTIME:</span> <span>02:14:55</span>
          </div>
        </div>
        <div className="flex space-x-2">
          <div className="px-3 py-1 border border-edge text-[9px] text-zinc-500 font-mono bg-black/50">THREAT_INTEL: REALTIME</div>
          <div className="px-3 py-1 bg-cyan-950 text-accent text-[9px] font-bold border border-accent/20 font-mono">SYSTEM_READY</div>
        </div>
      </footer>
    </div>
  );
};
