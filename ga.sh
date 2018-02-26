set -euxo pipefail

GA_CODE=$(cat <<EOF
<!-- Start Google Analytics -->
<script>
  (function(i,s,o,g,r,a,m){i['GoogleAnalyticsObject']=r;i[r]=i[r]||function(){
  (i[r].q=i[r].q||[]).push(arguments)},i[r].l=1*new Date();a=s.createElement(o),
  m=s.getElementsByTagName(o)[0];a.async=1;a.src=g;m.parentNode.insertBefore(a,m)
  })(window,document,'script','https://www.google-analytics.com/analytics.js','ga');

  ga('create', 'UA-87779174-1', 'auto');
  ga('send', 'pageview');

</script>
<!-- End Google Analytics -->
EOF
)

GA_CODE=$(echo $GA_CODE | sed -e 's/\n//g')

for f in $(find book -name '*.html'); do
    echo $f
    sed -i -e "s@\(</head>\)@$GA_CODE\n\1@" $f
done
