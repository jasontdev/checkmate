type ActivityViewProps = {
  activity: Activity;
};

function ActivityView({ activity }: ActivityViewProps) {
  return <div>{activity.project}</div>;
}

export default ActivityView;
