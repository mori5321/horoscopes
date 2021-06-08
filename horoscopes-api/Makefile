# 開発サーバー起動
up:
	docker-compose -f docker-compose.dev.yml up

down:
	docker-compose -f docker-compose.dev.yml down

sh:
	docker-compose -f docker-compose.dev.yml exec app bash

db:
	docker-compose -f docker-compose.dev.yml exec db psql -U root -d horoscopes_development

db.sh:
	docker-compose -f docker-compose.dev.yml exec db bash

migration.run:
	docker-compose -f docker-compose.dev.yml exec app diesel migration run

migration.revert:
	docker-compose -f docker-compose.dev.yml exec app diesel migration revert

# 仮置き
build.release:
	# docker run --rm -it -v `pwd`:./build/
	docker build -t horoscopes-api -f ./build/Dockerfile.Release .

run.release:
	docker run --rm --name horoscopes -p 3030:3030 --tty horoscopes-api

# cp:
# 	docker cp horoscopes:/app/target/release/horoscopes-api ./horoscopes-api


# build.release: イメージ作成
# -> run: コンテナ起動
# -> cp: 起動中のコンテナからシングルバイナリをもらってくる
