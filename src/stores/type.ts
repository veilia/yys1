export interface Act {
    id: string | null
    name: string
    cost: number
    tag: string
}

export interface ResAct {
    cnt: number
    t_cost: number
}

export interface Rec {
    id: string
    name: string
}

export interface ResRec {
    id: string
    num: number
}

export interface ResStat {
    name: string
    num: number
}