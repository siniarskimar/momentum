export interface Event {
  title: string;
  description: string;
  tags: string[];
  date: number;
  end_date: number;
  periodic_interval: number | null;
}
