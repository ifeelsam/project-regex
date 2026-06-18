import { invoke } from '@tauri-apps/api/core';

export type Platform = 'instagram' | 'x' | 'tiktok' | 'youtube' | 'web' | 'manual';
export type ItemStatus = 'inbox' | 'brewing' | 'ready' | 'producing' | 'shipped' | 'archived';
export type CapturedOn = 'desktop' | 'mobile';
export type ProjectFormat = 'video' | 'article' | 'other';

export type Item = {
  id: string;
  url: string | null;
  platform: Platform;
  title: string;
  author: string;
  note: string;
  develop_note: string;
  thumbnail_path: string | null;
  status: ItemStatus;
  project_id: string | null;
  captured_at: string;
  captured_on: CapturedOn;
};

export type Tag = {
  id: string;
  name: string;
};

export type Project = {
  id: string;
  title: string;
  brief: string;
  format: ProjectFormat;
  status: string;
  created_at: string;
  shipped_at: string | null;
};

export type CaptureItemInput = {
  url?: string | null;
  platform: Platform;
  title?: string | null;
  author?: string | null;
  note: string;
  tags: string[];
  captured_on: CapturedOn;
};

export type CaptureItemResult = {
  item: Item;
  created: boolean;
  tags: Tag[];
};

export type ItemWithTags = {
  item: Item;
  tags: Tag[];
};

export type SearchHit = {
  item: Item;
  rank: number;
  snippet: string;
  tags: Tag[];
};

export type IdeaDetail = {
  item: Item;
  tags: Tag[];
  references: ItemWithTags[];
};

function humanError(error: unknown): string {
  if (typeof error === 'string') return error;
  if (error && typeof error === 'object' && 'message' in error) {
    return String((error as { message: unknown }).message);
  }
  return 'Something went wrong. Try again.';
}

async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw new Error(humanError(error));
  }
}

export const api = {
  ping: () => call<string>('ping'),
  captureItem: (input: CaptureItemInput) => call<CaptureItemResult>('capture_item', { input }),
  enrichItem: (itemId: string) => call<void>('enrich_item', { item_id: itemId }),
  listInboxItems: (status?: ItemStatus) =>
    call<ItemWithTags[]>('list_inbox_items', { status: status ?? null }),
  listDevelopItems: () => call<ItemWithTags[]>('list_develop_items'),
  getIdeaDetail: (id: string) => call<IdeaDetail>('get_idea_detail', { id }),
  updateDevelopNote: (id: string, developNote: string) =>
    call<Item>('update_develop_note', { id, develop_note: developNote }),
  addItemReference: (ideaId: string, referenceId: string) =>
    call<void>('add_item_reference', { idea_id: ideaId, reference_id: referenceId }),
  removeItemReference: (ideaId: string, referenceId: string) =>
    call<void>('remove_item_reference', { idea_id: ideaId, reference_id: referenceId }),
  listItems: (status?: ItemStatus) => call<Item[]>('list_items', { status: status ?? null }),
  getItem: (id: string) => call<Item | null>('get_item', { id }),
  updateItemNote: (id: string, note: string) => call<Item>('update_item_note', { id, note }),
  updateItemStatus: (id: string, status: ItemStatus) =>
    call<Item>('update_item_status', { id, status }),
  deleteItem: (id: string) => call<void>('delete_item', { id }),
  listTags: () => call<Tag[]>('list_tags'),
  setItemTags: (itemId: string, tags: string[]) =>
    call<Tag[]>('set_item_tags', { item_id: itemId, tags }),
  searchItems: (query: string, limit = 50) =>
    call<SearchHit[]>('search_items', { query, limit }),
  createProject: (title: string, brief: string, format: ProjectFormat) =>
    call<Project>('create_project', { title, brief, format }),
  listProjects: () => call<Project[]>('list_projects'),
  graduateItem: (itemId: string, title: string, brief: string, format: ProjectFormat) =>
    call<Project>('graduate_item', {
      item_id: itemId,
      title,
      brief,
      format
    }),
  detectPlatform: (url: string) => call<Platform>('detect_platform', { url }),
  defaultCapturedOn: () => call<CapturedOn>('default_captured_on')
};
