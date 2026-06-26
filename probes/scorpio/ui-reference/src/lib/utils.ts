/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function formatTimestamp(date: Date = new Date()) {
  return date.toISOString().replace('T', ' ').substring(0, 19);
}

export function generateId() {
  return Math.random().toString(36).substring(2, 11);
}
