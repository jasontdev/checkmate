type ActivityViewProps = {
  activity: Activity;
};

function ActivityView({ activity }: ActivityViewProps) {
  return <div>{activity.project.name}</div>;
}

export default ActivityView;
