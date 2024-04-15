type numeric = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9";

/**
 * 日付を表す型
 */
export type Date = string;
	//`${numeric***REMOVED***${numeric***REMOVED***${numeric***REMOVED***${numeric***REMOVED***-${numeric***REMOVED***${numeric***REMOVED***-${numeric***REMOVED***${numeric***REMOVED***`;

/**
 * ユーザ情報
 */
export interface User {
	/** Ethereumのウォレットアドレス */
	id: string;

	/** PGrit ID */
	pgrit_id: string;
***REMOVED***

/**
 * 性別
 */
export type Sex = "Male" | "Female";

/**
 * 学生情報を表すインターフェース
 */
export interface Student {
	/** Ethereumのウォレットアドレス */
	user_id: string;
	/** 遂行中の学位 */
	degree_step: Degree;
	/** 学年 */
	grade: number;
	/** 受講コース */
	course: string;
	/** レベル */
	level: Level;
	/** 性別 */
	sex: Sex;
	/** 参加日 */
	join_date: Date;
	/** オフィス */
	office: string;
	/** メールアドレス */
	email: string;
	/** 4nonomeメールアドレス */
	email_of_4nonome: string;
	/** 大学 */
	university: string;
	/** 専攻 */
	major: string;
	/** 脱退日（オプション） */
	leave_date?: Date;
	/** アクティブ状態 */
	active: boolean;
	/** Slack ID */
	slack_id: string;
	/** Discord ID（オプション） */
	discord_id?: string;
***REMOVED***

/**
 * 取得遂行中の学位
 */
export type Degree = "Bachelor" | "Master" | "Doctor";

/**
 * PlayGroundのレベル
 */
export type Level = "新人" | "アシスタント" | "ノーマル" | "リード";

/**
 * PGNのレベル
 */
export type PgnLevel =
	| "Iron"
	| "Bronze"
	| "Silver"
	| "Gold"
	| "Platinum"
	| "Diamond"
	| "Master"
	| "GrandMaster";

/**
 * PgnSubstructの構造体定義
 */
export interface PgnInfo<T extends PgnLevel = PgnLevel> {
	/** PIXデータの更新日時 */
	updated_at: string;

	/** 1日ごとのPIX推移 */
	daily: { [date: Date]: number ***REMOVED***

	/** 現在のPgnLevel */
	level: T;

	/** 最近1ヶ月のPIX */
	last_month: number;

	/** 現在のレベルをベースにしたPIX */
	on_level: number;

	/** 現在のPgnLevelのステップがPIXいくつ分か */
	level_length: T extends "GrandMaster" ? undefined : number;

	/** 現在のPgnLevelでの進捗 */
	progress: T extends "GrandMaster" ? undefined : number;

	/** 次のレベルの月間総PIX */
	target: T extends "GrandMaster" ? undefined : number;

	/** 次のレベルまでに必要な残りのPIX */
	behind_next: T extends "GrandMaster" ? undefined : number;
***REMOVED***

/**
 * ユーザプロフィール
 */
export default interface UserProfile {
	/** ユーザ情報 */
	user: User;

	/**
	 * 学生情報
	 * 学生でない場合はundefined
	 */
	student?: Student;

	/** 作成日時 */
	created_at: Date;

	/** PGN情報 */
	pgn: PgnInfo;
***REMOVED***
