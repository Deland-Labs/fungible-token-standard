# find all *.sh in the current directory and change the permissions of all files to +x
#
ls -1 *.sh | xargs chmod -x
ls -1 *.sh | xargs git update-index --chmod=+x
ls -1 *.sh | xargs chmod +x
echo "All *.sh files in the current directory have been changed to +x"
