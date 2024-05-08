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

 Date: 06/05/2024 17:48:45
*/


-- ----------------------------
-- Table structure for node_info
-- ----------------------------
DROP TABLE IF EXISTS "public"."node_info";
CREATE TABLE "public"."node_info" (
  "node_id" text COLLATE "pg_catalog"."default" NOT NULL,
  "neighbor_nodes" text[] COLLATE "pg_catalog"."default" NOT NULL,
  "is_alive" bool NOT NULL
)
;
ALTER TABLE "public"."node_info" OWNER TO "postgres";

-- ----------------------------
-- Records of node_info
-- ----------------------------
BEGIN;
INSERT INTO "public"."node_info" VALUES ('0x679320A64036b710371374aEdfa59Cff5c16f1CA', '{0x679320A64036b710371374aEdfa59Cff5c16f1CB,0x679320A64036b710371374aEdfa59Cff5c16f1CC}', 't');
INSERT INTO "public"."node_info" VALUES ('0x679320A64036b710371374aEdfa59Cff5c16f1CB', '{0x679320A64036b710371374aEdfa59Cff5c16f1CA,0x679320A64036b710371374aEdfa59Cff5c16f1CC}', 't');
INSERT INTO "public"."node_info" VALUES ('0x679320A64036b710371374aEdfa59Cff5c16f1CC', '{0x679320A64036b710371374aEdfa59Cff5c16f1CA,0x679320A64036b710371374aEdfa59Cff5c16f1CB}', 't');
COMMIT;

-- ----------------------------
-- Primary Key structure for table node_info
-- ----------------------------
ALTER TABLE "public"."node_info" ADD CONSTRAINT "node_info_pkey" PRIMARY KEY ("node_id");
