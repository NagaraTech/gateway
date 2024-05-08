/*
 Navicat Premium Data Transfer

 Source Server         : localPostgre
 Source Server Type    : PostgreSQL
 Source Server Version : 130012
 Source Host           : 127.0.0.1:5432
 Source Catalog        : gateway
 Source Schema         : public

 Target Server Type    : PostgreSQL
 Target Server Version : 130012
 File Encoding         : 65001

 Date: 06/05/2024 17:48:35
*/


-- ----------------------------
-- Table structure for merge_log
-- ----------------------------
DROP TABLE IF EXISTS "public"."merge_log";
CREATE TABLE "public"."merge_log" (
  "id" int4 NOT NULL,
  "node_id" char(20) COLLATE "pg_catalog"."default" NOT NULL,
  "from_id" char(20) COLLATE "pg_catalog"."default",
  "to_id" char(20) COLLATE "pg_catalog"."default",
  "start_count" int4 NOT NULL,
  "end_count" int4 NOT NULL,
  "s_clock_hash" char(20) COLLATE "pg_catalog"."default" NOT NULL,
  "e_clock_hash" char(20) COLLATE "pg_catalog"."default" NOT NULL,
  "merge_at" timestamp(6) NOT NULL
)
;
ALTER TABLE "public"."merge_log" OWNER TO "postgres";

-- ----------------------------
-- Records of merge_log
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Primary Key structure for table merge_log
-- ----------------------------
ALTER TABLE "public"."merge_log" ADD CONSTRAINT "merge_log_pkey" PRIMARY KEY ("id");
