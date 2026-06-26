/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useRef, useEffect } from 'react';
import { motion } from 'motion/react';
import { AuditLog } from '@/src/types';
import { cn } from '@/src/lib/utils';

interface NerveCenterProps {
  logs: AuditLog[];
}

export const NerveCenter: React.FC<NerveCenterProps> = ({ logs }) => {
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [logs]);

  return (
    <div className="flex flex-col h-full bg-void border border-edge rounded-sm overflow-hidden">
      <div className="bg-panel px-4 py-2 border-b border-edge flex items-center justify-between">
        <div className="flex items-center space-x-3">
          <div className="w-1.5 h-1.5 rounded-full bg-accent shadow-[0_0_5px_#00F0FF]" />
          <span className="text-[10px] font-mono font-bold text-accent uppercase tracking-[0.3em] italic">Stream of Thought :: Augmented Opcode</span>
        </div>
      </div>
      
      <div 
        ref={scrollRef}
        className="flex-1 overflow-y-auto p-4 font-mono text-[10px] leading-relaxed space-y-1 selection:bg-accent/30 scroll-smooth"
      >
        {logs.map((log, index) => (
          <motion.div
            key={log.id}
            initial={{ opacity: 0, x: -4 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.1 }}
            className="flex space-x-4 group"
          >
            <span className="text-zinc-600 shrink-0 select-none">[{log.timestamp.split(' ')[1].substring(0, 5)}]</span>
            <span className={cn(
              "shrink-0 font-bold tracking-tighter w-24",
              log.agent === 'SENTINEL-ALPHA' ? "text-accent" : "text-indigo-400"
            )}>
              {log.agent}
            </span>
            <span className={cn(
               "flex-1",
               log.type === 'EXPLOIT' ? "text-orange-400 bg-orange-950/20 px-1" : 
               log.type === 'RESULT' ? "text-white glow-accent font-bold" : "text-zinc-500"
            )}>
              {log.message}
            </span>
            {log.metadata && (
                <span className="text-[9px] text-zinc-700 italic group-hover:text-zinc-500 transition-colors uppercase pr-2">
                  // {JSON.stringify(log.metadata).replace(/["{}]/g, '')}
                </span>
            )}
          </motion.div>
        ))}
        {logs.length === 0 && (
          <div className="flex items-center justify-center h-full text-zinc-800 italic flex-col space-y-4">
             <div className="w-8 h-8 border border-zinc-900 border-t-accent rounded-full animate-spin" />
             <span className="uppercase tracking-[0.5em] text-[8px]">Synchronizing Attention cycle...</span>
          </div>
        )}
      </div>
    </div>
  );
};
