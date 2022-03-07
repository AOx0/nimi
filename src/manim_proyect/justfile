rf:
    just render manim.cfg 2160p60

rl:
    just render manim_low.cfg 720p30

rvl:
    just render manim_very_low.cfg 360p15

lrvl:
    just render_local manim_very_low.cfg 360p15


lrf:
    just render_local manim.cfg 2160p60

lrf_web:
    just render_local_web manim.cfg 2160p60

render_local_web config quality:
    manim \
        --format webm \
        -c cfg/{{config}} \
        --disable_caching \
        main.py
    just openr {{quality}}

render_local config quality:
    manim \
        --format mp4 \
        -c cfg/{{config}} \
        --disable_caching \
        main.py
    just openr {{quality}}

render config quality:
    sshpass -p "1234" ssh parallels@192.168.0.15 \
        "\
            cd ~/Desktop/Parallels\ Shared\ Folders/Home/manim_test \
            && /home/parallels/.local/bin/manim \
                --format mp4 \
                -c cfg/{{config}} \
                main.py\
        "
    just openr {{quality}}

openr quality:
    qil QuickTime
    sleep 1
    open /Users/alejandro/manim_test/media/videos/main/{{quality}}/DefaultTemplate.mp4 -a "QuickTime PLayer"
