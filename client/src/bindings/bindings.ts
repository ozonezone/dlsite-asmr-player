/* eslint-disable */
// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "config.getScandir", input: never, result: string[] } | 
        { key: "ping", input: never, result: string } | 
        { key: "ping_auth", input: never, result: string } | 
        { key: "product.browse", input: BrowseParams, result: [ProductResult[], number] },
    mutations: 
        { key: "config.setPassword", input: string, result: string } | 
        { key: "config.setScandir", input: string[], result: string } | 
        { key: "scan.start", input: never, result: null },
    subscriptions: never
};

export type SortOrder = "Asc" | "Desc"

export type UserGenre = { id: string; name: string; count: number }

export type ProductResult = { id: string; name: string; description: string; series: string; circle_id: string; actor: string[]; author: string[]; illustrator: string[]; price: number; sale_count: number; age: Age; released_at: string; rating: number; rating_count: number; comment_count: number; path: string; remote_image: string[]; circle_name: string; genre: Genre[]; user_genre: UserGenre[] }

export type Age = "AllAges" | "R" | "Adult"

export type Genre = { id: string; name: string }

export type SortType = "Name" | "Date"

export type BrowseParams = { sort_type: SortType; sort_order: SortOrder; page: number; limit: number }
