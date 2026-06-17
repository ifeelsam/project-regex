export type NavItem = {
  href: string;
  label: string;
  description: string;
};

export const navItems: NavItem[] = [
  {
    href: '/inbox/',
    label: 'Inbox',
    description: 'Capture sparks and find anything you saved.'
  },
  {
    href: '/develop/',
    label: 'Develop',
    description: 'Think through ideas before you commit to producing them.'
  },
  {
    href: '/projects/',
    label: 'Projects',
    description: 'Ideas you have graduated into active production.'
  },
  {
    href: '/library/',
    label: 'Library',
    description: 'Frames, clips, audio, and transcripts from breakdowns.'
  },
  {
    href: '/settings/',
    label: 'Settings',
    description: 'Preferences, storage, and personal-use notice.'
  }
];
